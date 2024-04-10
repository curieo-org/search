use crate::err::AppError;
use crate::secrets::Secret;
use axum::extract::FromRef;
use color_eyre::eyre::eyre;
use redis::AsyncCommands;
use redis::Client as RedisClient;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct CacheSettings {
    pub cache_url: Secret<String>,
    pub enabled: bool,
    pub ttl: u64,
    pub max_sorted_size: i64,
}

#[derive(Clone, Debug, FromRef)]
pub struct Cache {
    settings: CacheSettings,
    client: RedisClient,
}

impl Cache {
    async fn cache_connect(cache_url: &str) -> Result<RedisClient, AppError> {
        match RedisClient::open(cache_url) {
            Ok(client) => Ok(client),
            Err(e) => Err(eyre!("Failed to connect to Redis: {}", e).into()),
        }
    }

    pub async fn new(cache_settings: &CacheSettings) -> Result<Self, AppError> {
        let client = Self::cache_connect(cache_settings.cache_url.expose()).await?;

        Ok(Self {
            settings: cache_settings.clone(),
            client,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.try_get(key).await {
            Ok(response) => response,
            Err(e) => {
                tracing::error!("Failed to get cache key {}: {}", key, e);
                None
            }
        }
    }

    pub async fn try_get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, AppError> {
        if !self.settings.enabled {
            return Ok(None);
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        let cache_response: Option<T> =
            connection.get(key).await.map(|response: Option<String>| {
                response.and_then(|response| serde_json::from_str(&response).ok())
            })?;

        Ok(cache_response)
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) {
        if let Err(e) = self.try_set(key, value).await {
            tracing::error!("Failed to set cache key {}: {}", key, e);
        }
    }

    pub async fn try_set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), AppError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        connection
            .set(
                key,
                serde_json::to_string(value)
                    .map_err(|_| eyre!("unable to convert string to json"))?,
            )
            .await?;

        Ok(())
    }

    pub async fn zincr(&self, space: &str, key: &str, value: i64) -> Result<(), AppError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        connection.zincr(space, key, value).await?;

        Ok(())
    }

    pub async fn zrevrange(
        &self,
        space: &str,
        start: i64,
        stop: i64,
    ) -> Result<Vec<String>, AppError> {
        if !self.settings.enabled {
            return Ok(vec![]);
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        let cache_response: Vec<String> = connection
            .zrevrange(space, start as isize - 1, stop as isize - 1)
            .await?;

        Ok(cache_response)
    }

    pub async fn zremrangebyrank(&self, space: &str) -> Result<(), AppError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        connection
            .zremrangebyrank(space, 0, -self.settings.max_sorted_size as isize - 1)
            .await?;

        Ok(())
    }
}

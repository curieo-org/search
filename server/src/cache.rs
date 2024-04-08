use crate::err::AppError;
use axum::extract::FromRef;
use color_eyre::eyre::eyre;
use redis::AsyncCommands;
use redis::Client as RedisClient;
use serde::{Deserialize, Serialize};
use std::future::Future;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct CacheSettings {
    pub cache_url: String,
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
        let client = Self::cache_connect(&cache_settings.cache_url).await?;

        Ok(Self {
            settings: cache_settings.clone(),
            client,
        })
    }

    pub async fn zincr(&self, space: &str, key: &str, value: i64) -> Result<(), AppError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        connection
            .zincr(space, key, value)
            .await
            .map_err(|e| AppError::from(e))?;

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
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(cache_response)
    }

    pub async fn zremrangebyrank(&self, space: &str) -> Result<(), AppError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        connection
            .zremrangebyrank(space, 0, -self.settings.max_sorted_size as isize - 1)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(())
    }
}

pub trait CacheFn<T> {
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<T>, AppError>>;
    fn set(&self, key: &str, value: &T) -> impl Future<Output = Result<(), AppError>>;
}

impl<T: Serialize + for<'de> Deserialize<'de>> CacheFn<T> for Cache {
    async fn get(&self, key: &str) -> Result<Option<T>, AppError> {
        if !self.settings.enabled {
            return Ok(None);
        }

        let mut connection = self.client.get_multiplexed_tokio_connection().await?;

        let cache_response: Option<T> = connection
            .get(key)
            .await
            .map(|response: Option<String>| {
                response.and_then(|response| serde_json::from_str(&response).ok())
            })
            .map_err(|e| AppError::from(e))?;

        Ok(cache_response)
    }

    async fn set(&self, key: &str, value: &T) -> Result<(), AppError> {
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
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(())
    }
}

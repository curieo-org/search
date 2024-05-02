use crate::err::AppError;
use crate::secrets::Secret;
use axum::extract::FromRef;
use bb8::Pool;
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, SetOptions};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct CacheSettings {
    pub url: Secret<String>,
    pub enabled: bool,
    pub ttl: u64,
    pub max_sorted_size: i64,
}

type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone, Debug, FromRef)]
pub struct CachePool {
    settings: CacheSettings,
    pool: RedisPool,
}

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("BB8 error: {0}")]
    BB8(#[from] bb8::RunError<redis::RedisError>),
    #[error("Cache deserialize error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl From<CacheError> for AppError {
    fn from(e: CacheError) -> Self {
        AppError::Cache(e)
    }
}

impl CachePool {
    pub async fn new(cache_settings: &CacheSettings) -> Result<Self, CacheError> {
        tracing::debug!("Connecting to redis");
        let manager = RedisConnectionManager::new(cache_settings.url.expose().as_str())?;
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(2))
            .build(manager)
            .await?;

        let cache = Self {
            settings: cache_settings.clone(),
            pool,
        };

        if cache_settings.enabled {
            if let Err(e) = cache.verify().await {
                tracing::error!("Failed to connect to redis: {}", e);
            }
        }

        Ok(cache)
    }

    async fn verify(&self) -> Result<(), CacheError> {
        let mut conn = self.pool.get().await?;
        conn.set("ping", "pong").await?;
        let result: String = conn.get("ping").await?;
        assert_eq!(result, "pong");
        tracing::debug!("Successfully connected to redis and pinged it");
        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.try_get(key).await {
            Ok(response) => response,
            Err(e) => {
                tracing::error!("Failed to get cache key '{}': {}", key, e);
                None
            }
        }
    }

    pub async fn try_get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            let cache_response: Option<T> =
                conn.get(key).await.map(|response: Option<String>| {
                    response.and_then(|response| serde_json::from_str(&response).ok())
                })?;
            return Ok(cache_response);
        }
        Ok(None)
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) {
        if let Err(e) = self.try_set(key, value).await {
            tracing::error!("Failed to set cache key {}: {}", key, e);
        }
    }

    pub async fn try_set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            conn.set(key, serde_json::to_string(value)?).await?;
        }
        Ok(())
    }

    pub async fn set_options<T: Serialize>(&self, key: &str, value: &T, opts: SetOptions) {
        if let Err(e) = self.try_set_options(key, value, opts).await {
            tracing::error!("Failed to set cache key {}: {}", key, e);
        }
    }

    pub async fn try_set_options<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        opts: SetOptions,
    ) -> Result<(), CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            conn.set_options(key, serde_json::to_string(value)?, opts)
                .await?;
        }
        Ok(())
    }

    pub async fn del(&self, key: &str) -> Result<(), CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            conn.del(key).await?;
        }
        Ok(())
    }

    pub async fn zincr(&self, space: &str, key: &str, value: i64) -> Result<(), CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            conn.zincr(space, key, value).await?;
        }
        Ok(())
    }

    pub async fn zrevrange(
        &self,
        space: &str,
        start: i64,
        stop: i64,
    ) -> Result<Vec<String>, CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            let result: Vec<String> = conn
                .zrevrange(space, start as isize - 1, stop as isize - 1)
                .await?;
            return Ok(result);
        }
        Ok(vec![])
    }

    pub async fn zremrangebyrank(&self, space: &str) -> Result<(), CacheError> {
        if let Ok(mut conn) = self.pool.get().await {
            conn.zremrangebyrank(space, 0, -self.settings.max_sorted_size as isize - 1)
                .await?;
        }
        Ok(())
    }
}

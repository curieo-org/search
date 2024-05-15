use crate::cache::{CacheError, CachePool};
use async_trait::async_trait;
use axum_login::tower_sessions::session::{Id, Record};
use axum_login::tower_sessions::{session_store, SessionStore};
use redis::{ExistenceCheck, SetExpiry, SetOptions};
use std::fmt::Debug;
use time::OffsetDateTime;

impl From<CacheError> for session_store::Error {
    fn from(err: CacheError) -> Self {
        match err {
            CacheError::Cache(inner) => session_store::Error::Backend(inner.to_string()),
            CacheError::BB8(inner) => session_store::Error::Backend(inner.to_string()),
            CacheError::Serde(inner) => session_store::Error::Decode(inner.to_string()),
        }
    }
}

/// A Redis session store.
#[derive(Debug, Clone)]
pub struct RedisStore {
    pool: CachePool,
}

impl RedisStore {
    pub fn new(pool: CachePool) -> Self {
        Self { pool }
    }

    async fn save_with_options(
        &self,
        record: &Record,
        options: Option<SetOptions>,
    ) -> session_store::Result<()> {
        let options = options.unwrap_or_else(|| {
            SetOptions::default()
                .conditional_set(ExistenceCheck::NX)
                .get(true)
                .with_expiration(SetExpiry::EXAT(
                    OffsetDateTime::unix_timestamp(record.expiry_date) as usize,
                ))
        });

        self.pool
            .try_set_options(record.id.to_string().as_str(), record, options)
            .await
            .map_err(session_store::Error::from)
    }
}

#[async_trait]
impl SessionStore for RedisStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        loop {
            if self
                .save_with_options(
                    record,
                    Some(SetOptions::default().conditional_set(ExistenceCheck::NX)),
                )
                .await
                .is_err()
            {
                record.id = Id::default();
                continue;
            }
            break;
        }
        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.save_with_options(
            record,
            Some(SetOptions::default().conditional_set(ExistenceCheck::XX)),
        )
        .await?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        Ok(self
            .pool
            .try_get::<Record>(session_id.to_string().as_str())
            .await?)
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        Ok(self.pool.del(session_id.to_string().as_str()).await?)
    }
}

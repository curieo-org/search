use crate::cache::{CacheError, CachePool};
use async_trait::async_trait;
use axum_login::tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};
use dashmap::DashMap;
use redis::{ExistenceCheck, SetExpiry, SetOptions};
use std::sync::Arc;
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

#[derive(Clone, Debug, Default)]
pub struct DashStore(Arc<DashMap<Id, Record>>);

#[async_trait]
impl SessionStore for DashStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        while self.0.contains_key(&record.id) {
            // Session ID collision mitigation.
            record.id = Id::default();
        }
        self.0.insert(record.id, record.clone());
        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.0.insert(record.id, record.clone());
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        Ok(self.0.get(session_id).and_then(|r| {
            let val = r.value();
            if is_active(val.expiry_date) {
                Some(val.clone())
            } else {
                None
            }
        }))
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        self.0.remove(session_id);
        Ok(())
    }
}

fn is_active(expiry_date: OffsetDateTime) -> bool {
    expiry_date > OffsetDateTime::now_utc()
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_create() {
        let store = DashStore::default();
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        assert!(store.create(&mut record).await.is_ok());
    }

    #[tokio::test]
    async fn test_save() {
        let store = DashStore::default();
        let record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        assert!(store.save(&record).await.is_ok());
    }

    #[tokio::test]
    async fn test_load() {
        let store = DashStore::default();
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        let loaded_record = store.load(&record.id).await.unwrap();
        assert_eq!(Some(record), loaded_record);
    }

    #[tokio::test]
    async fn test_delete() {
        let store = DashStore::default();
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        assert!(store.delete(&record.id).await.is_ok());
        assert_eq!(None, store.load(&record.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_id_collision() {
        let store = DashStore::default();
        let expiry_date = OffsetDateTime::now_utc() + Duration::minutes(30);
        let mut record1 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        let mut record2 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        store.create(&mut record1).await.unwrap();
        record2.id = record1.id; // Set the same ID for record2
        store.create(&mut record2).await.unwrap();
        assert_ne!(record1.id, record2.id); // IDs should be different
    }
}

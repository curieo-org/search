use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::fmt::Debug;
use time;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(#[serde(with = "time::serde::rfc3339")] pub time::OffsetDateTime);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SourceType {
    PDF,
    Image,
    URL,
}

impl From<i32> for SourceType {
    fn from(value: i32) -> Self {
        match value {
            0 => SourceType::PDF,
            1 => SourceType::Image,
            2 => SourceType::URL,
            _ => SourceType::URL,
        }
    }
}

// impl To<i32> for SourceType {
//     fn to(&self) -> i32 {
//         match self {
//             SourceType::PDF => 0,
//             SourceType::Image => 1,
//             SourceType::URL => 2,
//         }
//     }
// }

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Thread {
    pub thread_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub context: Option<serde_json::Value>,

    // pub created_at: DateTime,
    // pub updated_at: DateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Search {
    pub search_id: uuid::Uuid,
    pub thread_id: uuid::Uuid,
    pub query: String,
    pub result: String,
    pub media_urls: Option<Vec<String>>,
    pub reaction: Option<bool>,

    // pub created_at: DateTime,
    // pub updated_at: DateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    pub source_id: uuid::Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub source_type: SourceType,
    pub metadata: Option<serde_json::Value>,

    // pub created_at: DateTime,
    // pub updated_at: DateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SearchSource {
    pub search_source_id: uuid::Uuid,
    pub search_id: uuid::Uuid,
    pub source_id: uuid::Uuid,

    // pub created_at: DateTime,
    // pub updated_at: DateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

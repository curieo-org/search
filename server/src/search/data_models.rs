// use crate::proto::Source;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::collections::HashMap;
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

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Thread {
    pub thread_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub context: Option<serde_json::Value>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Search {
    pub search_id: uuid::Uuid,
    pub thread_id: uuid::Uuid,
    pub query: String,
    pub result: String,
    pub media_urls: Option<Vec<String>>,
    pub reaction: Option<bool>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    pub source_id: uuid::Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub source_type: SourceType,
    pub metadata: Option<HashMap<String, String>>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

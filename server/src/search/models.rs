use serde::{Deserialize, Serialize};
use sqlx::types::time;
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQueryRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchHistoryRequest {
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub response_text: String,
    pub response_sources: Vec<String>,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SearchHistory {
    pub search_history_id: uuid::Uuid,
    // pub user_id: uuid::Uuid,
    pub search_text: String,
    pub response_text: String,
    pub response_sources: Vec<String>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

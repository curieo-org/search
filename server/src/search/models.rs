use serde::{Deserialize, Serialize};
use sqlx::types::time;
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct TopSearchRequest {
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RAGTokenResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQueryRequest {
    pub query: String,
    pub session_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchHistoryRequest {
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchReactionRequest {
    pub search_history_id: uuid::Uuid,
    pub reaction: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub result: String,
    pub sources: Vec<String>,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SearchHistory {
    pub search_history_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub query: String,
    pub result: String,
    pub sources: Vec<String>,
    pub reaction: Option<bool>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

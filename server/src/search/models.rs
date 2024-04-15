use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::types::time;
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct TopSearchRequest {
    pub limit: Option<i64>,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BraveSource {
    pub url: String,
    pub page_age: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
pub enum Source {
    Brave(BraveSource),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SourceOld {
    pub origin: String,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Sources(pub Vec<Source>);

impl From<serde_json::Value> for Sources {
    fn from(value: serde_json::Value) -> Sources {
        match value {
            serde_json::Value::Array(array) => {
                let mut sources = vec![];
                for v in array {
                    match serde_json::from_value(v) {
                        Ok(source) => sources.push(source),
                        Err(e) => tracing::error!("Failed to parse source: {:?}", e),
                    }
                }
                Self(sources)
            }
            _ => {
                tracing::error!("Failed to parse sources: {:?}", value);
                Sources(vec![])
            }
        }
    }
}

impl TryFrom<&Sources> for serde_json::Value {
    type Error = serde_json::Error;

    fn try_from(sources: &Sources) -> Result<Self, Self::Error> {
        serde_json::to_value(sources)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub result: String,
    pub sources: Sources,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SearchHistory {
    pub search_history_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub query: String,
    pub result: String,
    pub sources: Sources,
    pub reaction: Option<bool>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

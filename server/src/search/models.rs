use crate::proto::{Metadata, Source};
use serde::{Deserialize, Serialize};
use sqlx::types::time;
use sqlx::types::JsonValue;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchSource(pub Vec<Source>);

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SearchHistory {
    pub search_history_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub query: String,
    pub result: String,
    pub sources: SearchSource,
    pub reaction: Option<bool>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl From<JsonValue> for SearchSource {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Array(arr) => {
                let mut sources = Vec::new();
                for source in arr {
                    let json = source.as_object().unwrap();
                    let url = json.get("url").unwrap().as_str().unwrap();
                    let metadata = json.get("metadata").unwrap().as_array().unwrap();
                    let mut metadata_vec = Vec::new();

                    for item in metadata {
                        let json = item.as_object().unwrap();
                        let key = json.get("key").unwrap().as_str().unwrap();
                        let value = json.get("value").unwrap().as_str().unwrap();
                        metadata_vec.push(Metadata {
                            key: key.to_string(),
                            value: value.to_string(),
                        });
                    }

                    sources.push(Source {
                        url: url.to_string(),
                        metadata: metadata_vec,
                    });
                }
                SearchSource(sources)
            }
            _ => SearchSource(Vec::new()),
        }
    }
}

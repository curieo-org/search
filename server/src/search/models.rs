use crate::proto::{Pair, Source};
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
pub struct SearchSource(Vec<Source>);

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
                    let pairs = json.get("pairs").unwrap().as_array().unwrap();
                    let mut pairs_vec = Vec::new();

                    for pair in pairs {
                        let json = pair.as_object().unwrap();
                        let key = json.get("key").unwrap().as_str().unwrap();
                        let value = json.get("value").unwrap().as_str().unwrap();
                        pairs_vec.push(Pair {
                            key: key.to_string(),
                            value: value.to_string(),
                        });
                    }

                    sources.push(Source { pairs: pairs_vec });
                }
                SearchSource(sources)
            }
            _ => SearchSource(Vec::new()),
        }
    }
}

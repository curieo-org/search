use crate::search::SourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    pub url: String,
    pub title: String,
    pub description: String,
    pub source_type: SourceType,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedResult {
    pub text: String,
    pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub result: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSettings {
    pub max_query_length: u16,
    pub max_sources: u8,
    pub max_search_context: u8,
}

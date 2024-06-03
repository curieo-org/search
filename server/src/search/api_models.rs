use crate::search::{Source, Thread};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub enum RouteCategory {
    ResearchArticle = 0,
    ClinicalTrials = 1,
    Drug = 2,
    NotSpecified = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQueryRequest {
    pub query: String,
    pub thread_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchByIdResponse {
    pub result: String,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchHistoryRequest {
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchHistoryResponse {
    pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchThreadRequest {
    pub thread_id: uuid::Uuid,
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchThreadResponse {
    pub searches: Vec<SearchByIdResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchByIdRequest {
    pub search_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchReactionRequest {
    pub search_id: uuid::Uuid,
    pub reaction: bool,
}

//#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
//pub struct BraveSource {
//    pub url: String,
//    pub page_age: Option<String>,
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
//#[serde(tag = "type")]
//pub enum Source {
//    Brave(BraveSource),
//}

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct Sources(pub Vec<Source>);

// impl From<serde_json::Value> for Sources {
//     fn from(value: serde_json::Value) -> Self {
//         if let serde_json::Value::Array(array) = value {
//             Sources(
//                 array
//                     .into_iter()
//                     .filter_map(|v| serde_json::from_value(v).ok())
//                     .collect(),
//             )
//         } else {
//             tracing::warn!("Invalid SearchSource: {:?}", value);
//             Sources(vec![])
//         }
//     }
// }

// impl TryFrom<&Sources> for serde_json::Value {
//     type Error = serde_json::Error;

//     fn try_from(sources: &Sources) -> Result<Self, Self::Error> {
//         serde_json::to_value(sources)
//     }
// }

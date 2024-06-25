use crate::search::Search;
use crate::search::{Source, Thread};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type BoxedFuture = Pin<Box<dyn Future<Output = crate::Result<Search>> + Send>>;

pub struct UpdateResultProcessor {
    pub processor: Arc<dyn Fn(String) -> BoxedFuture + Send + Sync>,
}

impl UpdateResultProcessor {
    pub fn new(processor: Arc<dyn Fn(String) -> BoxedFuture + Send + Sync>) -> Self {
        UpdateResultProcessor { processor }
    }

    pub async fn process(&self, result: String) -> crate::Result<Search> {
        let search = (self.processor)(result).await?;
        Ok(search)
    }
}

impl Debug for UpdateResultProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UpdateResultProcessor")
    }
}

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
    pub search: Search,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadHistoryRequest {
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadHistoryResponse {
    pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetThreadRequest {
    pub thread_id: uuid::Uuid,
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchThreadResponse {
    pub thread: Thread,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateThreadRequest {
    pub thread_id: uuid::Uuid,
    pub title: String,
}

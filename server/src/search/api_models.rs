use crate::err::ErrorExt;
use crate::search::Search;
use crate::search::{Source, Thread};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use validator::Validate;

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
    ResearchArticle,
    ClinicalTrials,
    Drug,
    NotSpecified,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SearchQueryRequest {
    #[validate(length(min = 1, max = 300))]
    pub query: String,
    pub thread_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchByIdResponse {
    pub search: Search,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ThreadHistoryRequest {
    #[validate(range(min = 1, max = 20))]
    pub limit: Option<u8>,
    #[validate(range(min = 0))]
    pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadHistoryResponse {
    pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct GetThreadRequest {
    pub thread_id: uuid::Uuid,
    #[validate(range(min = 1, max = 20))]
    pub limit: Option<u8>,
    #[validate(range(min = 0))]
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

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateThreadRequest {
    pub thread_id: uuid::Uuid,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
}

#[derive(Debug)]
pub enum SearchError {
    ToxicQuery(String),
    InvalidQuery(String),
    AgencyFailure(String),
    LLMFailure(String),
    BraveFailure(String),
    StreamFailure(String),
    NoResults(String),
    NoSources(String),
}

impl ErrorExt for SearchError {
    fn to_error_code(&self) -> String {
        match self {
            SearchError::ToxicQuery(_) => format!("toxic_query"),
            SearchError::InvalidQuery(_) => format!("invalid_query"),
            SearchError::AgencyFailure(_) => format!("agency_failure"),
            SearchError::LLMFailure(_) => format!("llm_failure"),
            SearchError::BraveFailure(_) => format!("brave_failure"),
            SearchError::StreamFailure(_) => format!("stream_failure"),
            SearchError::NoResults(_) => format!("no_results"),
            SearchError::NoSources(_) => format!("no_sources"),
        }
    }

    fn to_status_code(&self) -> StatusCode {
        match self {
            SearchError::ToxicQuery(_) => StatusCode::UNPROCESSABLE_ENTITY,
            SearchError::InvalidQuery(_) => StatusCode::UNPROCESSABLE_ENTITY,
            SearchError::AgencyFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::LLMFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::BraveFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::StreamFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::NoResults(_) => StatusCode::NOT_FOUND,
            SearchError::NoSources(_) => StatusCode::NOT_FOUND,
        }
    }
}

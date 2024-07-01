use crate::search::{Search, Source, Thread};
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc};
use tonic::Status as TonicStatus;
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

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    Reqwest(#[from] reqwest::Error),
    ReqwestHeaderName(#[from] InvalidHeaderName),
    ReqwestHeaderValue(#[from] InvalidHeaderValue),
    Serde(#[from] SerdeError),
    Tonic(#[from] TonicStatus),
    ToxicQuery(String),
    InvalidQuery(String),
    AgencyFailure(String),
    NoResults(String),
    NoSources(String),
    Other(String),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SearchError: {}", self)
    }
}

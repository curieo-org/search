use crate::proto::{
    agency_service_client::AgencyServiceClient, Embeddings, PubmedResponse, PubmedSource,
};
use crate::rag::{RetrievedResult, Source};
use crate::search::{SearchError, SourceType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubmedSettings {
    pub url_prefix: String,
}

pub fn convert_to_retrieved_result(
    pubmed_settings: &PubmedSettings,
    source: PubmedSource,
) -> RetrievedResult {
    RetrievedResult {
        text: source.r#abstract.clone(),
        source: Source {
            url: format!("{}/{}", pubmed_settings.url_prefix, source.pubmed_id),
            title: source.title,
            description: source.r#abstract,
            source_type: SourceType::Url,
            metadata: HashMap::new(),
        },
    }
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn pubmed_parent_search(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    embeddings: &Embeddings,
) -> Result<Vec<PubmedSource>, SearchError> {
    let request = tonic::Request::new(embeddings.clone());
    let mut agency_service = agency_service.as_ref().clone();

    let response: PubmedResponse = agency_service
        .pubmed_parent_search(request)
        .await?
        .into_inner();

    if response.status != 200 {
        return Err(SearchError::AgencyFailure(
            "Failed to get pubmed parent search results".to_string(),
        ));
    }

    Ok(response.sources)
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn pubmed_cluster_search(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    embeddings: &Embeddings,
) -> Result<Vec<PubmedSource>, SearchError> {
    let request = tonic::Request::new(embeddings.clone());
    let mut agency_service = agency_service.as_ref().clone();

    let response: PubmedResponse = agency_service
        .pubmed_cluster_search(request)
        .await?
        .into_inner();

    if response.status != 200 {
        return Err(SearchError::AgencyFailure(
            "Failed to get pubmed cluster search results".to_string(),
        ));
    }

    Ok(response.sources)
}

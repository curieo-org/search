use crate::proto::agency_service_client::AgencyServiceClient;
use crate::proto::{PubmedResponse, PubmedSource, SearchRequest};
use crate::rag::{RetrievedResult, Source};
use crate::search::SourceType;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubmedSettings {
    pub url_prefix: String,
}

fn convert_to_retrieved_result(
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

#[tracing::instrument(level = "debug", ret, err)]
pub async fn pubmed_parent_search(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    pubmed_settings: &PubmedSettings,
    search_query: &str,
) -> crate::Result<Vec<RetrievedResult>> {
    let request = tonic::Request::new(SearchRequest {
        query: search_query.to_string(),
    });
    let mut agency_service = agency_service.as_ref().clone();

    let response: PubmedResponse = agency_service
        .pubmed_parent_search(request)
        .await
        .map_err(|e| eyre!("Request to agency failed: {e}"))?
        .into_inner();

    if response.status != 200 {
        return Err(eyre!("Failed to get search results").into());
    }

    let retrieved_results: Vec<RetrievedResult> = response
        .sources
        .into_iter()
        .map(|source| convert_to_retrieved_result(pubmed_settings, source))
        .collect();

    Ok(retrieved_results)
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn pubmed_cluster_search(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    pubmed_settings: &PubmedSettings,
    search_query: &str,
) -> crate::Result<Vec<RetrievedResult>> {
    let request = tonic::Request::new(SearchRequest {
        query: search_query.to_string(),
    });
    let mut agency_service = agency_service.as_ref().clone();

    let response: PubmedResponse = agency_service
        .pubmed_cluster_search(request)
        .await
        .map_err(|e| eyre!("Request to agency failed: {e}"))?
        .into_inner();

    if response.status != 200 {
        return Err(eyre!("Failed to get search results").into());
    }

    let retrieved_results: Vec<RetrievedResult> = response
        .sources
        .into_iter()
        .map(|source| convert_to_retrieved_result(pubmed_settings, source))
        .collect();

    Ok(retrieved_results)
}

use crate::cache::CachePool;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag::{self, post_process};
use crate::rag::{brave_search, pubmed_search};
use crate::search::api_models;
use crate::settings::Settings;
use std::sync::Arc;
use tonic::transport::Channel;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    settings: &Settings,
    brave_api_config: &brave_search::BraveAPIConfig,
    cache: &CachePool,
    agency_service: &mut AgencyServiceClient<Channel>,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<rag::SearchResponse> {
    if let Some(response) = cache.get(&search_query_request.query).await {
        return Ok(response);
    }

    let agency_service = Arc::new(agency_service.clone());
    let retrieved_results = retrieve_results(
        settings,
        brave_api_config,
        agency_service,
        search_query_request,
    )
    .await?;

    let response = post_process::post_process_search_results(
        &settings.llm,
        search_query_request,
        retrieved_results,
    )
    .await?;

    cache.set(&search_query_request.query, &response).await;

    return Ok(response);
}

#[tracing::instrument(level = "debug", ret, err)]
async fn retrieve_results(
    settings: &Settings,
    brave_api_config: &brave_search::BraveAPIConfig,
    agency_service: Arc<AgencyServiceClient<Channel>>,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<Vec<rag::RetrievedResult>> {
    let (pubmed_parent_response, pubmed_cluster_response, brave_response) = tokio::join!(
        pubmed_search::pubmed_parent_search(
            Arc::clone(&agency_service),
            &settings.pubmed,
            &search_query_request.query
        ),
        pubmed_search::pubmed_cluster_search(
            Arc::clone(&agency_service),
            &settings.pubmed,
            &search_query_request.query
        ),
        brave_search::web_search(
            &settings.brave,
            brave_api_config,
            &search_query_request.query
        ),
    );

    let mut all_retrieved_results: Vec<rag::RetrievedResult> = Vec::new();

    if let Ok(response) = pubmed_parent_response {
        all_retrieved_results.extend(response);
    }

    if let Ok(response) = pubmed_cluster_response {
        all_retrieved_results.extend(response);
    }

    if let Ok(response) = brave_response {
        all_retrieved_results.extend(response);
    }

    Ok(all_retrieved_results)
}

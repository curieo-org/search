use crate::cache::CachePool;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag::{self, post_process, pre_process};
use crate::rag::{brave_search, pubmed_search};
use crate::search::api_models;
use crate::settings::Settings;
use std::sync::Arc;
use tonic::transport::Channel;
use crate::llms::toxicity_llm;
use color_eyre::eyre::eyre;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    settings: &Settings,
    brave_api_config: &brave_search::BraveAPIConfig,
    cache: &CachePool,
    agency_service: &mut AgencyServiceClient<Channel>,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<rag::SearchResponse> {
    let toxicity_prediction = toxicity_llm::predict_toxicity(
        &settings.llm,
        toxicity_llm::ToxicityInput{
        inputs: search_query_request.query.to_string(),
    }).await?;

    if toxicity_prediction {
        return Err(eyre!("Query is too toxic").into());
    }

    if let Some(response) = cache.get(&search_query_request.query).await {
        return Ok(response);
    }

    let (agency_results, fallback_results) = tokio::join!(
        retrieve_result_from_agency(
            settings,
            agency_service,
            &search_query_request,
        ),
        brave_search::web_search(
            &settings.brave,
            brave_api_config,
            &search_query_request.query,
        ),
    );

    let mut retrieved_results = Vec::new();
    if let Ok(agency_results) = agency_results {
        retrieved_results.extend(agency_results);
    }
    if let Ok(fallback_results) = fallback_results {
        retrieved_results.extend(fallback_results);
    }

    let response = post_process::rerank_search_results(
        &settings.llm,
        search_query_request,
        retrieved_results,
    )
    .await?;

    cache.set(&search_query_request.query, &response).await;

    return Ok(response);
}

#[tracing::instrument(level = "debug", ret, err)]
async fn retrieve_result_from_agency(
    settings: &Settings,
    agency_service: &mut AgencyServiceClient<Channel>,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<Vec<rag::RetrievedResult>> {
    let agency_service = Arc::new(agency_service.clone());
    let embeddings =
        pre_process::compute_embeddings(Arc::clone(&agency_service), &search_query_request.query)
            .await?;

    let (pubmed_parent_response, pubmed_cluster_response) = tokio::join!(
        pubmed_search::pubmed_parent_search(
            Arc::clone(&agency_service),
            &settings.pubmed,
            &embeddings,
        ),
        pubmed_search::pubmed_cluster_search(
            Arc::clone(&agency_service),
            &settings.pubmed,
            &embeddings,
        ),
    );

    let mut all_retrieved_results: Vec<rag::RetrievedResult> = Vec::new();

    if let Ok(response) = pubmed_parent_response {
        all_retrieved_results.extend(response);
    }
    if let Ok(response) = pubmed_cluster_response {
        all_retrieved_results.extend(response);
    }

    Ok(all_retrieved_results)
}

use crate::llms::query_rephraser;
use crate::proto::{
    agency_service_client::AgencyServiceClient, Embeddings, EmbeddingsOutput, SearchInput,
};
use crate::search::{api_models, services as search_services, SearchError};
use crate::settings::Settings;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::transport::Channel;

#[tracing::instrument(level = "info", ret, err)]
pub async fn compute_embeddings(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    search_query: &str,
) -> Result<Embeddings, SearchError> {
    let request = tonic::Request::new(SearchInput {
        query: search_query.to_string(),
    });
    let mut agency_service = agency_service.as_ref().clone();

    let response: EmbeddingsOutput = agency_service
        .embeddings_compute(request)
        .await?
        .into_inner();

    if response.status != 200 {
        return Err(SearchError::AgencyFailure("Failed to get embeddings".to_string()).into());
    }

    match response.embeddings {
        None => Err(SearchError::AgencyFailure("No embeddings found".to_string()).into()),
        Some(embeddings) => Ok(embeddings),
    }
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn rephrase_query(
    pool: &PgPool,
    settings: &Settings,
    search_query_request: &api_models::SearchQueryRequest,
) -> Result<String, SearchError> {
    let last_n_searches = match search_query_request.thread_id {
        Some(thread_id) => search_services::get_last_n_searches(
            &pool,
            settings.search.max_search_context,
            &thread_id,
        )
        .await
        .map_err(|e| SearchError::Other(format!("Failed to get last n searches: {}", e)))?,
        None => vec![],
    };

    if last_n_searches.is_empty() {
        return Ok(search_query_request.query.clone());
    }

    let rephraser_response = query_rephraser::rephrase_query(
        &settings.query_rephraser,
        &query_rephraser::QueryRephraserInput {
            query: search_query_request.query.clone(),
            previous_context: last_n_searches
                .into_iter()
                .map(|s| query_rephraser::QueryResult {
                    query: s.rephrased_query,
                    result: s.result,
                })
                .collect(),
        },
    )
    .await?;

    Ok(rephraser_response
        .rephrased_query
        .chars()
        .take(settings.search.max_query_length as usize)
        .collect())
}

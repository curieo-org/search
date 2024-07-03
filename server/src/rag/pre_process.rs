use crate::llms::query_rephraser;
use crate::llms::toxicity;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::proto::{Embeddings, EmbeddingsOutput, SearchInput};
use crate::search::api_models;
use crate::search::services as search_services;
use crate::settings::Settings;
use color_eyre::eyre::eyre;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::transport::Channel;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn compute_embeddings(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    search_query: &str,
) -> crate::Result<Embeddings> {
    let request = tonic::Request::new(SearchInput {
        query: search_query.to_string(),
    });
    let mut agency_service = agency_service.as_ref().clone();

    let response: EmbeddingsOutput = agency_service
        .embeddings_compute(request)
        .await
        .map_err(|e| eyre!("Request to agency failed: {e}"))?
        .into_inner();

    if response.status != 200 {
        return Err(eyre!("Failed to get search results").into());
    }

    match response.embeddings {
        None => Err(eyre!("No embeddings found").into()),
        Some(embeddings) => Ok(embeddings),
    }
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn check_query_validity(
    settings: &Settings,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<bool> {
    if search_query_request.query.len() > settings.search.max_query_length as usize {
        return Ok(false);
    }

    let toxicity_prediction = toxicity::predict_toxicity(
        &settings.llm,
        toxicity::ToxicityInput {
            inputs: search_query_request.query.to_string(),
        },
    )
    .await?;

    Ok(!toxicity_prediction)
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rephrase_query(
    pool: &PgPool,
    settings: &Settings,
    search_query_request: &api_models::SearchQueryRequest,
) -> crate::Result<String> {
    let last_n_searches = match search_query_request.thread_id {
        Some(thread_id) => {
            search_services::get_last_n_searches(
                pool,
                settings.search.max_search_context,
                &thread_id,
            )
            .await?
        }
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

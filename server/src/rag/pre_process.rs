use crate::llms::toxicity_llm;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::proto::{Embeddings, EmbeddingsOutput, SearchInput};
use crate::search::api_models;
use crate::settings::Settings;
use color_eyre::eyre::eyre;
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
    if search_query_request.query.len() > settings.max_search_query_length as usize {
        return Ok(false);
    }

    let toxicity_prediction = toxicity_llm::predict_toxicity(
        &settings.llm,
        toxicity_llm::ToxicityInput {
            inputs: search_query_request.query.to_string(),
        },
    )
    .await?;

    Ok(!toxicity_prediction)
}

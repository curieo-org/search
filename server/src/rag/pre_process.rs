use crate::proto::agency_service_client::AgencyServiceClient;
use crate::proto::{Embeddings, EmbeddingsOutput, SearchInput};
use color_eyre::eyre::eyre;
use std::sync::Arc;
use tonic::transport::Channel;
use crate::settings::Settings;
use crate::llms::toxicity_llm;

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
async fn preprocess_query(
    agency_service: Arc<AgencyServiceClient<Channel>>,
    settings: &Settings,
    search_query: &str,
) -> crate::Result<Embeddings> {
    let (toxicity_prediction, embeddings) = tokio::join!(
        toxicity_llm::predict_toxicity(
            &settings.llm,
            toxicity_llm::ToxicityInput{
            inputs: search_query.to_string(),
        }),
        compute_embeddings(agency_service, search_query)
    );

    let toxicity_prediction = toxicity_prediction?;
    let embeddings = embeddings?;

    if toxicity_prediction {
        return Err(eyre!("Query is too toxic").into());
    }
    Ok(embeddings)
}

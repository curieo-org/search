use crate::llms::summarizer;
use crate::llms::SummarizerSettings;
use crate::proto::Embeddings;
use crate::rag::utils;
use crate::search::api_models;
use std::cmp::Ordering;
use tokio::sync::mpsc::Sender;

#[tracing::instrument(level = "debug", ret)]
pub fn rerank_search_results(
    query_embeddings: &Embeddings,
    results_embeddings: &Vec<Embeddings>,
) -> Vec<usize> {
    let query_dense_embedding = &query_embeddings.dense_embedding;

    let mut cosine_similarities: Vec<(usize, f64)> = results_embeddings
        .iter()
        .enumerate()
        .map(|(index, result_embeddings)| {
            let result_dense_embedding = &result_embeddings.dense_embedding;
            let cosine_similarity =
                utils::cosine_similarity(query_dense_embedding, result_dense_embedding);
            (index, cosine_similarity)
        })
        .collect();

    cosine_similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    cosine_similarities
        .iter()
        .map(|(index, _)| *index)
        .collect()
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn summarize_search_results(
    settings: SummarizerSettings,
    search_query_request: api_models::SearchQueryRequest,
    search_response: String,
    update_processor: api_models::UpdateResultProcessor,
    tx: Sender<api_models::SearchByIdResponse>,
) -> crate::Result<()> {
    summarizer::generate_text_stream(
        settings,
        summarizer::SummarizerInput {
            query: search_query_request.query,
            retrieved_result: search_response,
        },
        update_processor,
        tx,
    )
    .await?;

    Ok(())
}

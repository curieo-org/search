use crate::llms::summarizer;
use crate::proto::Embeddings;
use crate::rag::utils;
use crate::search::api_models;
use crate::settings::Settings;
use regex::Regex;
use std::cmp::Ordering;
use tokio::sync::mpsc::Sender;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rerank_search_results(
    query_embeddings: &Embeddings,
    results_embeddings: &Vec<Embeddings>,
) -> crate::Result<Vec<usize>> {
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

    let result: Vec<usize> = cosine_similarities
        .iter()
        .map(|(index, _)| *index)
        .collect();

    return Ok(result);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn summarize_search_results(
    settings: Settings,
    search_query_request: api_models::SearchQueryRequest,
    search_response: String,
    update_processor: api_models::UpdateResultProcessor,
    stream_regex: Regex,
    tx: Sender<api_models::SearchByIdResponse>,
) -> crate::Result<()> {
    summarizer::generate_text_with_openai(
        settings.openai,
        summarizer::SummarizerInput {
            query: search_query_request.query,
            retrieved_result: search_response,
        },
        update_processor,
        stream_regex,
        tx,
    )
    .await?;

    Ok(())
}

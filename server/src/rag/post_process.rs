use crate::llms::{llm_lingua, summarizer};
use crate::llms::{LLMSettings, SummarizerSettings};
use crate::rag::RetrievedResult;
use crate::rag::SearchResponse;
use crate::search::api_models;
use tokio::sync::mpsc::Sender;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rerank_search_results(
    llm_settings: &LLMSettings,
    search_query_request: &api_models::SearchQueryRequest,
    retrieved_results: Vec<RetrievedResult>,
) -> crate::Result<SearchResponse> {
    let compressed_results = llm_lingua::compress_and_rerank(
        llm_settings,
        llm_lingua::LlmLinguaInput {
            query: search_query_request.query.clone(),
            target_token: 300,
            context_texts_list: retrieved_results.iter().map(|r| r.text.clone()).collect(),
        },
    )
    .await?;

    let mut reranked_sources = Vec::new();
    for index in compressed_results.sources {
        reranked_sources.push(retrieved_results[index as usize].source.clone());
    }
    reranked_sources.truncate(llm_settings.top_k_sources as usize);

    Ok(SearchResponse {
        result: compressed_results.compressed_prompt,
        sources: reranked_sources,
    })
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn summarize_search_results(
    settings: SummarizerSettings,
    search_query_request: api_models::SearchQueryRequest,
    search_response: String,
    update_processor: api_models::UpdateResultProcessor,
    tx: Sender<SearchResponse>,
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

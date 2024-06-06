use crate::llms::LLMSettings;
use crate::llms::{bio_llm, llm_lingua};
use crate::rag::RetrievedResult;
use crate::rag::SearchResponse;
use crate::search::api_models;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn post_process_search_results(
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

    let summaiized_results = bio_llm::generate_text(
        llm_settings,
        bio_llm::BioLLMInput {
            query: search_query_request.query.clone(),
            retrieved_result: compressed_results.compressed_prompt,
        },
    )
    .await?;

    let response = SearchResponse {
        result: summaiized_results.generated_text,
        sources: reranked_sources,
    };

    Ok(response)
}

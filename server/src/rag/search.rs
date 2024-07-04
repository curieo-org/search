use crate::cache::CachePool;
use crate::llms::prompt_compression;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag::{self, post_process, pre_process};
use crate::rag::{brave_search, pubmed_search};
use crate::settings::Settings;
use color_eyre::eyre::eyre;
use std::sync::Arc;
use tonic::transport::Channel;

#[tracing::instrument(level = "info", ret, err)]
pub async fn search(
    settings: &Settings,
    brave_api_config: &brave_search::BraveAPIConfig,
    cache: &CachePool,
    agency_service: &AgencyServiceClient<Channel>,
    search_query: &str,
) -> crate::Result<rag::SearchResponse> {
    if let Some(response) = cache.get(search_query).await {
        return Ok(response);
    }

    let (agency_results, fallback_results) = tokio::join!(
        retrieve_result_from_agency(settings, agency_service, search_query),
        brave_search::web_search(&settings.brave, brave_api_config, search_query),
    );

    let mut retrieved_results = Vec::new();
    if let Ok(agency_results) = agency_results {
        retrieved_results.extend(agency_results);
    }

    let max_sources = settings.search.max_sources as usize;
    if retrieved_results.len() < max_sources {
        if let Ok(fallback_results) = fallback_results {
            let required_results_count = fallback_results
                .len()
                .min(max_sources - retrieved_results.len());
            retrieved_results.extend(fallback_results.into_iter().take(required_results_count));
        }
    }

    if retrieved_results.is_empty() {
        return Err(eyre!("No results found").into());
    }

    let compressed_results = prompt_compression::compress(
        &settings.llm,
        prompt_compression::PromptCompressionInput {
            query: search_query.to_string(),
            target_token: 300,
            context_texts_list: retrieved_results.iter().map(|r| r.text.clone()).collect(),
        },
    )
    .await?;

    let response = rag::SearchResponse {
        result: compressed_results.compressed_prompt,
        sources: retrieved_results.into_iter().map(|r| r.source).collect(),
    };
    cache.set(search_query, &response).await;

    return Ok(response);
}

#[tracing::instrument(level = "info", ret, err)]
async fn retrieve_result_from_agency(
    settings: &Settings,
    agency_service: &AgencyServiceClient<Channel>,
    search_query: &str,
) -> crate::Result<Vec<rag::RetrievedResult>> {
    let agency_service = Arc::new(agency_service.clone());
    let query_embeddings =
        pre_process::compute_embeddings(Arc::clone(&agency_service), search_query).await?;

    let (pubmed_parent_response, pubmed_cluster_response) = tokio::join!(
        pubmed_search::pubmed_parent_search(Arc::clone(&agency_service), &query_embeddings),
        pubmed_search::pubmed_cluster_search(Arc::clone(&agency_service), &query_embeddings),
    );

    let mut retrieved_results: Vec<rag::RetrievedResult> = Vec::new();
    let mut source_embeddings = Vec::new();

    if let Ok(response) = pubmed_parent_response {
        response.into_iter().for_each(|source| {
            if let Some(embeddings) = source.embeddings.clone() {
                retrieved_results.push(pubmed_search::convert_to_retrieved_result(
                    &settings.pubmed,
                    source,
                ));

                source_embeddings.push(embeddings);
            }
        });
    }

    if let Ok(response) = pubmed_cluster_response {
        response.into_iter().for_each(|source| {
            if let Some(embeddings) = source.embeddings.clone() {
                retrieved_results.push(pubmed_search::convert_to_retrieved_result(
                    &settings.pubmed,
                    source,
                ));

                source_embeddings.push(embeddings);
            }
        });
    }

    let reranked_indices =
        post_process::rerank_search_results(&query_embeddings, &source_embeddings).await?;

    let top_k = reranked_indices
        .len()
        .min(settings.search.max_sources as usize);
    let reranked_retrieved_results = reranked_indices
        .into_iter()
        .take(top_k)
        .map(|index| retrieved_results[index].clone())
        .collect();

    Ok(reranked_retrieved_results)
}

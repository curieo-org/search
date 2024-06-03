use crate::cache::CachePool;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::proto::{SearchRequest as AgencySearchRequest, SearchResponse as AgencySearchResponse};
use crate::search::api_models;
use color_eyre::eyre::eyre;
use tonic::transport::Channel;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    cache: &CachePool,
    agency_service: &mut AgencyServiceClient<Channel>,
    search_query: &api_models::SearchQueryRequest,
) -> crate::Result<AgencySearchResponse> {
    if let Some(response) = cache.get(&search_query.query).await {
        return Ok(response);
    }

    let request = tonic::Request::new(AgencySearchRequest {
        query: search_query.query.clone(),
    });

    let response: AgencySearchResponse = agency_service
        .pubmed_bioxriv_web_search(request)
        .await
        .map_err(|e| eyre!("Request to agency failed: {e}"))?
        .into_inner();

    if response.status != 200 {
        return Err(eyre!("Failed to get search results").into());
    }

    cache.set(&search_query.query, &response).await;

    return Ok(response);
}

use crate::cache::Cache;
use crate::err::AppError;
use crate::proto::rag_service_client::RagServiceClient;
use crate::proto::{RouteCategory, SearchRequest, SearchResponse};
use crate::search::{
    SearchHistory, SearchHistoryRequest, SearchQueryRequest, SearchReactionRequest,
    TopSearchRequest,
};
use color_eyre::eyre::eyre;
use rand::Rng;
use sqlx::PgPool;
use tonic::transport::Channel;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    cache: &Cache,
    rag_service: &mut RagServiceClient<Channel>,
    search_query: &SearchQueryRequest,
) -> crate::Result<SearchResponse> {
    if let Some(response) = cache.get(&search_query.query).await {
        return Ok(response);
    }

    let request = tonic::Request::new(SearchRequest {
        query: search_query.query.clone(),
        route_category: RouteCategory::PubmedBioxrivWeb as i32,
    });

    let response: SearchResponse = rag_service
        .search(request)
        .await
        .map_err(|_| eyre!("unable to send request to search service"))?
        .into_inner();

    if response.status != 200 {
        return Err(AppError::from(eyre!("failed to get search results")));
    }

    cache.set(&search_query.query, &response).await;

    return Ok(response);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_search_history(
    pool: &PgPool,
    cache: &Cache,
    user_id: &Uuid,
    search_query: &SearchQueryRequest,
    search_response: &SearchResponse,
) -> crate::Result<SearchHistory> {
    let session_id = search_query.session_id.unwrap_or(Uuid::new_v4());

    let search_history = sqlx::query_as!(
        SearchHistory,
        "insert into search_history (user_id, session_id, query, result, sources) values ($1, $2, $3, $4, $5) returning *",
        user_id,
        &session_id,
        search_query.query,
        search_response.result,
        serde_json::to_value(&search_response.sources).map_err(|_| eyre!("unable to serialize sources"))?
    )
    .fetch_one(pool)
    .await?;

    return Ok(search_history);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_search_history(
    pool: &PgPool,
    user_id: &Uuid,
    search_history_request: &SearchHistoryRequest,
) -> crate::Result<Vec<SearchHistory>> {
    let search_history = sqlx::query_as!(
        SearchHistory,
        "select * from search_history where user_id = $1 order by created_at desc limit $2 offset $3",
        user_id,
        search_history_request.limit.unwrap_or(10) as i64,
        search_history_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    return Ok(search_history);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_top_searches(
    cache: &Cache,
    top_search_request: &TopSearchRequest,
) -> crate::Result<Vec<String>> {
    let random_number = rand::thread_rng().gen_range(0.0..1.0);
    if random_number < 0.1 {
        cache.zremrangebyrank("search_history").await?;
    }

    let limit = top_search_request.limit.unwrap_or(10);
    if !(1..=100).contains(&limit) {
        Err(eyre!("limit must be a number between 1 and 100"))?;
    }

    let top_searches: Vec<String> = cache.zrevrange("search_history", 1, limit).await?;

    return Ok(top_searches);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_search_reaction(
    pool: &PgPool,
    user_id: &Uuid,
    search_reaction_request: &SearchReactionRequest,
) -> crate::Result<SearchHistory> {
    let search_history = sqlx::query_as!(
        SearchHistory,
        "update search_history set reaction = $1 where search_history_id = $2 and user_id = $3 returning *",
        search_reaction_request.reaction,
        search_reaction_request.search_history_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(search_history);
}

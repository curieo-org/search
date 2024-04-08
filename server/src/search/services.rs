use crate::cache::{Cache, CacheFn};
use crate::err::AppError;
use crate::search::{
    RAGTokenResponse, SearchHistory, SearchHistoryRequest, SearchQueryRequest, SearchResponse,
    TopSearchRequest,
};
use crate::settings::SETTINGS;
use color_eyre::eyre::eyre;
use rand::Rng;
use reqwest::Client as ReqwestClient;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    cache: &Cache,
    search_query: &SearchQueryRequest,
) -> crate::Result<SearchResponse> {
    let cache_response: Option<SearchResponse> = cache.get(&search_query.query).await?;
    if let Some(response) = cache_response {
        return Ok(response);
    }

    // TODO: replace this with actual search logic using GRPC calls with backend services
    let rag_api_url = SETTINGS.rag_api.clone() + "/token";
    let form_data = [
        ("username", &SETTINGS.rag_api_username.expose()),
        ("password", &SETTINGS.rag_api_password.expose()),
    ];
    let token: RAGTokenResponse = ReqwestClient::new()
        .post(rag_api_url)
        .form(&form_data)
        .send()
        .await
        .map_err(|_| eyre!("unable to send request to rag api"))?
        .json()
        .await
        .map_err(|_| eyre!("unable to parse json response from rag api"))?;

    let rag_api_url = SETTINGS.rag_api.clone() + "/search?query=" + &search_query.query;
    let response: SearchResponse = ReqwestClient::new()
        .get(rag_api_url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .map_err(|_| eyre!("unable to send request to rag api"))?
        .json()
        .await
        .map_err(|_| eyre!("unable to parse json response from rag api"))?;

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
    cache.set(&search_query.query, search_response).await?;

    let search_history = sqlx::query_as!(
        SearchHistory,
        "insert into search_history (user_id, query, result, sources) values ($1, $2, $3, $4) returning *",
        user_id,
        search_query.query,
        search_response.result,
        &search_response.sources
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::from(e))?;

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
    .await
    .map_err(|e| AppError::from(e))?;

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
    if limit < 1 || limit > 100 {
        Err(eyre!("limit must be a number between 1 and 100"))?;
    }

    let top_searches: Vec<String> = cache.zrevrange("search_history", 1, limit).await?;

    return Ok(top_searches);
}

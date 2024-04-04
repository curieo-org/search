use crate::err::AppError;
use crate::search::{SearchHistory, SearchQueryRequest, SearchResponse};
use color_eyre::eyre::eyre;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use sqlx::PgPool;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    cache: &mut MultiplexedConnection,
    search_query: &SearchQueryRequest,
) -> crate::Result<SearchResponse> {
    let cache_response: Option<String> = cache
        .get(search_query.query.clone())
        .await
        .map_err(|e| AppError::from(e))?;

    let cache_response = match cache_response {
        Some(response) => response,
        None => String::new(),
    };

    let cache_response: Option<SearchResponse> =
        serde_json::from_str(&cache_response).unwrap_or(None);

    if let Some(response) = cache_response {
        return Ok(response);
    }

    // sleep for 3 seconds to simulate a slow search
    // TODO: replace this with actual search logic using GRPC calls with backend services
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let response = SearchResponse {
        response_text: "sample response".to_string(),
        response_sources: vec!["www.source1.com".to_string(), "www.source2.com".to_string()],
    };

    return Ok(response);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_search_history(
    pool: &PgPool,
    cache: &mut MultiplexedConnection,
    search_query: &SearchQueryRequest,
    search_response: &SearchResponse,
) -> crate::Result<SearchHistory> {
    cache
        .set(
            &search_query.query,
            serde_json::to_string(&search_response)
                .map_err(|_| eyre!("unable to convert string to json"))?,
        )
        .await
        .map_err(|e| AppError::from(e))?;

    let search_history = sqlx::query_as!(
        SearchHistory,
        "insert into search_history (search_text, response_text, response_sources) values ($1, $2, $3) returning *",
        &search_query.query,
        &search_response.response_text,
        &search_response.response_sources
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::from(e))?;

    return Ok(search_history);
}

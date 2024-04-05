use crate::err::AppError;
use crate::users::User;
use crate::search::services;
use crate::search::{SearchHistory, SearchHistoryRequest, SearchQueryRequest, TopSearchRequest};
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use redis::{AsyncCommands, Client as RedisClient};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_handler(
    State(pool): State<PgPool>,
    State(cache): State<RedisClient>,
    user: User,
    Query(search_query): Query<SearchQueryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let mut connection = cache
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::from(e))?;

    let search_response = services::search(&mut connection, &search_query).await?;
    services::insert_search_history(
        &pool,
        &mut connection,
        &user_id,
        &search_query,
        &search_response,
    )
    .await?;

    connection
        .zincr("search_queries", &search_query.query, 1)
        .await
        .map_err(|e| AppError::from(e))?;

    Ok((StatusCode::OK, Json(search_response)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_history_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_history_request): Query<SearchHistoryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history = sqlx::query_as!(
        SearchHistory,
        "select * from search_history where user_id = $1 order by created_at desc limit $2 offset $3",
        user_id,
        search_history_request.limit.unwrap_or(10) as i64,
        search_history_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::from(e))?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_top_searches_handler(
    State(cache): State<RedisClient>,
    Query(query): Query<TopSearchRequest>,
) -> crate::Result<impl IntoResponse> {
    let mut connection = cache
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::from(e))?;

    let top_searches = services::get_top_searches(&mut connection, &query).await?;

    Ok((StatusCode::OK, Json(top_searches)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_handler))
        .route("/history", get(get_search_history_handler))
        .route("/top", get(get_top_searches_handler))
}

use crate::err::AppError;
use crate::search::services;
use crate::search::{SearchHistory, SearchHistoryRequest, SearchQueryRequest, TopSearchRequest};
use crate::settings::SETTINGS;
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use color_eyre::eyre::eyre;
use rand::Rng;
use redis::{AsyncCommands, Client as RedisClient};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_handler(
    State(pool): State<PgPool>,
    State(cache): State<RedisClient>,
    Query(search_query): Query<SearchQueryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str("78c4c766-f310-11ee-a6ee-5f4062fc15f2").unwrap();

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
    Query(search_history_request): Query<SearchHistoryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str("78c4c766-f310-11ee-a6ee-5f4062fc15f2").unwrap();

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

    let random_number = rand::thread_rng().gen_range(0.0..1.0);
    if random_number < 0.1 {
        connection
            .zremrangebyrank(
                "search_history",
                0,
                -SETTINGS.cache_max_sorted_size as isize - 1,
            )
            .await
            .map_err(|e| AppError::from(e))?;
    }

    let limit = query.limit.unwrap_or(10);
    if limit < 1 || limit > 100 {
        Err(eyre!("limit must be a number between 1 and 100"))?;
    }

    let top_searches: Vec<String> = connection
        .zrevrange("search_queries", 0, limit as isize - 1)
        .await
        .map_err(|e| AppError::from(e))?;

    Ok((StatusCode::OK, Json(top_searches)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_handler))
        .route("/history", get(get_search_history_handler))
        .route("/top", get(get_top_searches_handler))
}

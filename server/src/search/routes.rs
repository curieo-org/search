use crate::cache::CachePool;
use crate::search::services;
use crate::search::{
    SearchHistoryRequest, SearchQueryRequest, SearchReactionRequest, TopSearchRequest,
};
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch};
use axum::{Json, Router};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_handler(
    State(pool): State<PgPool>,
    State(cache): State<CachePool>,
    user: User,
    Query(search_query): Query<SearchQueryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_response = services::search(&cache, &search_query).await?;
    let search_history =
        services::insert_search_history(&pool, &cache, &user_id, &search_query, &search_response)
            .await?;

    cache
        .zincr("search_queries", &search_query.query, 1)
        .await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_history_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_history_request): Query<SearchHistoryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history =
        services::get_search_history(&pool, &user_id, &search_history_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_top_searches_handler(
    State(cache): State<CachePool>,
    Query(query): Query<TopSearchRequest>,
) -> crate::Result<impl IntoResponse> {
    let top_searches = services::get_top_searches(&cache, &query).await?;

    Ok((StatusCode::OK, Json(top_searches)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_search_reaction_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(search_reaction_request): Json<SearchReactionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history =
        services::update_search_reaction(&pool, &user_id, &search_reaction_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_handler))
        .route("/history", get(get_search_history_handler))
        .route("/top", get(get_top_searches_handler))
        .route("/reaction", patch(update_search_reaction_handler))
}

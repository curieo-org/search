use crate::cache::CachePool;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag;
use crate::search::{api_models, services};
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch};
use axum::{Json, Router};
use sqlx::PgPool;
use tonic::transport::Channel;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_query_handler(
    State(pool): State<PgPool>,
    State(cache): State<CachePool>,
    State(mut agency_service): State<AgencyServiceClient<Channel>>,
    user: User,
    Query(search_query): Query<api_models::SearchQueryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let (search_item, search_response) = tokio::join!(
        services::insert_new_search(&pool, &user_id, &search_query),
        rag::search(&cache, &mut agency_service, &search_query)
    );
    let search_item = search_item?;
    let search_response = search_response?;

    let search_history =
        services::update_search_result(&pool, &user_id, &search_item, &search_response).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_one_search_result_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_history_request): Query<api_models::SearchByIdRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history = services::get_one_search(&pool, &user_id, &search_history_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_threads_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_history_request): Query<api_models::SearchHistoryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history = services::get_threads(&pool, &user_id, &search_history_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_one_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_thread_request): Query<api_models::SearchThreadRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_thread = services::get_one_thread(&pool, &user_id, &search_thread_request).await?;

    Ok((StatusCode::OK, Json(search_thread)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_thread_request): Json<api_models::UpdateThreadRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::update_thread(&pool, &user_id, &update_thread_request).await?;

    Ok(StatusCode::OK)
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_search_reaction_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(search_reaction_request): Json<api_models::SearchReactionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::update_search_reaction(&pool, &user_id, &search_reaction_request).await?;

    Ok(StatusCode::OK)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_query_handler))
        .route("/one", get(get_one_search_result_handler))
        .route("/threads", get(get_one_thread_handler))
        .route("/threads", patch(update_thread_handler))
        .route("/history", get(get_threads_handler))
        .route("/reaction", patch(update_search_reaction_handler))
}

use crate::collections::{api_models, services, data_models};
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn create_collection_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(create_collections_request): Json<api_models::CreateCollectionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let collection = services::insert_new_collection(&pool, &user_id, &create_collections_request).await?;

    Ok((StatusCode::OK, Json(collection)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_collection_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_collection_request): Json<api_models::UpdateCollectionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let collection = services::update_collection(&pool, &user_id, &update_collection_request).await?;

    Ok((StatusCode::OK, Json(collection)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_collections_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(get_collections_request): Query<api_models::GetCollectionsRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let collections = services::get_collections(&pool, &user_id, &get_collections_request).await?;

    Ok((StatusCode::OK, Json(collections)))
}

pub fn routes() -> Router<AppState> {
  Router::new()
      .route("/", post(create_collection_handler))
      .route("/", patch(update_collection_handler))
      .route("/all", get(get_collections_handler))
}
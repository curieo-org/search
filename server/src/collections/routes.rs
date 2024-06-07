use crate::collections::{api_models, services, data_models};
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, put};
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
async fn delete_collection_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(delete_collection_request): Json<api_models::DeleteCollectionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::delete_collection(&pool, &user_id, &delete_collection_request).await?;

    Ok((StatusCode::OK, ()))
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

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn add_items_to_collection_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(add_items_to_collection_request): Json<api_models::AddItemsToCollectionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::add_items_to_collection(&pool, &user_id, &add_items_to_collection_request).await?;

    Ok((StatusCode::OK, ()))
}

pub fn routes() -> Router<AppState> {
  Router::new()
      .route("/", post(create_collection_handler))
      .route("/", patch(update_collection_handler))
      .route("/", delete(delete_collection_handler))
      .route("/all", get(get_collections_handler))
      .route("/items", put(add_items_to_collection_handler))
}
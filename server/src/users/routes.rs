use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::PgPool;

use crate::startup::AppState;
use crate::users::selectors::get_user;
use crate::users::User;

async fn get_user_handler(
    State(pool): State<PgPool>,
    Json(user_id): Json<uuid::Uuid>,
) -> crate::Result<Json<Option<User>>> {
    Ok(Json(get_user(pool, user_id).await?))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/user", get(get_user_handler))
}

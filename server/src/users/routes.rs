use crate::startup::AppState;
use crate::users::selectors::get_user;
use crate::users::User;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use color_eyre::eyre::eyre;
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<uuid::Uuid>,
) -> crate::Result<Json<User>> {
    match get_user(pool, user_id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(eyre!("User not found").into()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/:user_id", get(get_user_handler))
}

use crate::startup::AppState;
use crate::users::{User, UserRecord};
use axum::routing::get;
use axum::{Json, Router};

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_user_handler(user: User) -> crate::Result<Json<UserRecord>> {
    return Ok(Json(UserRecord::from(user)));
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/me", get(get_user_handler))
}

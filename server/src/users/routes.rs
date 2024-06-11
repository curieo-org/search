use crate::auth::{AuthSession, PasswordCredentials};
use crate::err::AppError;
use crate::startup::AppState;
use crate::users::UserRecord;
use axum::routing::get;
use axum::{Json, Router};

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_user_handler(auth_session: AuthSession) -> crate::Result<Json<UserRecord>> {
    let user = auth_session
        .user
        .map(UserRecord::from)
        .ok_or_else(|| AppError::Unauthorized)?;

    Ok(Json(user))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/me", get(get_user_handler))
}

use crate::auth::AuthSession;
use crate::auth::utils::verify_user_password;
use crate::err::AppError;
use crate::startup::AppState;
use crate::users::{User, UserRecord, models, services};
use color_eyre::eyre::eyre;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch};
use axum::extract::State;
use axum::{Form, Json, Router};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_user_handler(auth_session: AuthSession) -> crate::Result<Json<UserRecord>> {
    let user = auth_session
        .user
        .map(UserRecord::from)
        .ok_or_else(|| AppError::Unauthorized)?;

    Ok(Json(user))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_profile_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_profile_request): Json<models::UpdateProfileRequest>
) -> crate::Result<Json<UserRecord>>  {
    let user_id = user.user_id;
    if !update_profile_request.has_any_value() {
        return Err(eyre!("At least one field has to be updated.").into())
    }
    let updated_user = services::update_profile(&pool, &user_id, update_profile_request).await?;

    Ok(Json(UserRecord::from(updated_user)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_password_handler(
    State(pool): State<PgPool>,
    user: User,
    Form(update_password_request): Form<models::UpdatePasswordRequest>
) -> crate::Result<impl IntoResponse>  {
    let user_id = user.user_id;

    if update_password_request.old_password.expose() == update_password_request.new_password.expose() {
        return Err(eyre!("Old and new password can not be the same.").into());
    }

    match verify_user_password(Some(user), update_password_request.old_password) {
        Ok(Some(_user)) => {
            services::update_password(&pool, &user_id, update_password_request.new_password).await?;
            Ok((StatusCode::OK, ()))
        },
        _ => Err(eyre!("Failed to authenticate old password").into())
        
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_user_handler))
        .route("/me", patch(update_profile_handler))
        .route("/update-password", patch(update_password_handler))
}

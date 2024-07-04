use crate::auth::utils::verify_user_password;
use crate::startup::AppState;
use crate::users::{api_models, services, User, UserError, UserRecord};
use axum::routing::{get, patch};
use axum::{extract::State, Form, Json, Router};
use sqlx::PgPool;
use validator::Validate;

#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn get_user_handler(user: User) -> crate::Result<Json<UserRecord>> {
    Ok(Json(UserRecord::from(user)))
}

#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn update_profile_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_profile_request): Json<api_models::UpdateProfileRequest>,
) -> crate::Result<Json<UserRecord>> {
    update_profile_request
        .validate()
        .map_err(|e| UserError::InvalidData(format!("Invalid data: {}", e)))?;
    let user_id = user.user_id;
    if !update_profile_request.has_any_value() {
        return Err(
            UserError::InvalidData(format!("At least one field has to be updated.")).into(),
        );
    }
    let updated_user = services::update_profile(&pool, &user_id, update_profile_request).await?;

    Ok(Json(UserRecord::from(updated_user)))
}

#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn update_password_handler(
    State(pool): State<PgPool>,
    user: User,
    Form(update_password_request): Form<api_models::UpdatePasswordRequest>,
) -> crate::Result<()> {
    let user_id = user.user_id;

    if update_password_request.old_password.expose()
        == update_password_request.new_password.expose()
    {
        return Err(
            UserError::InvalidData(format!("Old and new password can not be the same.")).into(),
        );
    }

    match verify_user_password(Some(user), update_password_request.old_password) {
        Ok(Some(_user)) => {
            services::update_password(&pool, &user_id, update_password_request.new_password)
                .await?;
            Ok(())
        }
        _ => Err(UserError::InvalidPassword(format!("Failed to authenticate old password")).into()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_user_handler))
        .route("/me", patch(update_profile_handler))
        .route("/update-password", patch(update_password_handler))
}

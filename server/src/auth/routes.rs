use crate::auth::models::{AuthSession, Credentials, RegisterUserRequest};
use crate::auth::oauth_2::Issuer;
use crate::auth::services;
use crate::auth::{AccessTokenCredentials, OAuthCredentials, PasswordCredentials};
use crate::err::AppError;
use crate::startup::AppState;
use crate::users::UserRecord;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use axum_login::tower_sessions::Session;
use color_eyre::eyre::eyre;
use oauth2::CsrfToken;
use serde::Deserialize;
use sqlx::PgPool;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn register_handler(
    State(pool): State<PgPool>,
    Form(request): Form<RegisterUserRequest>,
) -> crate::Result<impl IntoResponse> {
    if !services::is_email_whitelisted(&pool, &request.email).await? {
        return Err(eyre!("This email is not whitelisted!").into());
    }
    services::register(pool, request)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
}

#[tracing::instrument(level = "debug", skip_all)]
async fn login_handler(
    mut auth_session: AuthSession,
    Form(creds): Form<PasswordCredentials>,
) -> crate::Result<Json<UserRecord>> {
    let user = match auth_session
        .authenticate(Credentials::Password(creds))
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(eyre!("Could not authenticate user").into()),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(eyre!("Could not login user").into());
    }
    Ok(Json(UserRecord::from(user)))
}

pub const CSRF_STATE_KEY: &str = "auth.csrf-state";
pub const NEXT_URL_KEY: &str = "auth.next-url";
pub const PKCE_VERIFIER_KEY: &str = "auth.pkce-verifier";

#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn oauth_callback_handler(
    mut auth_session: AuthSession,
    session: Session,
    Path(issuer): Path<Issuer>,
    Query(AuthzResp { code, state }): Query<AuthzResp>,
) -> crate::Result<Json<UserRecord>> {
    let Ok(Some(old_state)) = session.get(CSRF_STATE_KEY).await else {
        return Err(eyre!("Session did not contain old csrf state").into());
    };

    let creds = Credentials::OAuth(OAuthCredentials {
        issuer,
        code,
        old_state,
        new_state: state,
    });

    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(eyre!("Could not authenticate user").into()),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(eyre!("Could not login user").into());
    }

    Ok(Json(UserRecord::from(user)))
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn verify_access_token_handler(
    mut auth_session: AuthSession,
    Form(access_token_credentials): Form<AccessTokenCredentials>,
) -> crate::Result<Json<UserRecord>> {
    let creds = Credentials::AccessToken(access_token_credentials);
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(eyre!("Could not authenticate user").into()),
    };
    if auth_session.login(&user).await.is_err() {
        return Err(eyre!("Could not login user").into());
    }
    Ok(Json(UserRecord::from(user)))
}

#[tracing::instrument(level = "debug", skip_all)]
async fn logout_handler(mut auth_session: AuthSession) -> crate::Result<()> {
    auth_session
        .logout()
        .await
        .map_err(|e| eyre!("Failed to logout: {}", e))?;

    Ok(())
}

#[tracing::instrument(level = "debug", skip_all)]
async fn get_session_handler(auth_session: AuthSession) -> crate::Result<Json<UserRecord>> {
    auth_session
        .user
        .map(UserRecord::from)
        .map(Json::from)
        .ok_or_else(|| AppError::Unauthorized)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/oauth_callback/:issuer", get(oauth_callback_handler))
        .route("/logout", get(logout_handler))
        .route("/get-session", get(get_session_handler))
        .route("/callback/credentials", post(login_handler))
        .route("/verify_access_token", post(verify_access_token_handler))
}

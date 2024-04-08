use crate::auth::models::{AuthSession, Credentials, RegisterUserRequest};
use crate::auth::services::register;
use crate::auth::{OAuthCredentials, PasswordCredentials};
use crate::err::AppError;
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
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
    register(pool, request)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
}

#[tracing::instrument(level = "debug", skip_all)]
async fn login_handler(
    mut auth_session: AuthSession,
    Form(creds): Form<PasswordCredentials>,
) -> crate::Result<()> {
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
    //if let Credentials::Password(_pw_creds) = creds {
    //    if let Some(ref next) = pw_creds.next {
    //        return Redirect::to(next).into_response();
    //    }
    //}
    Ok(())
}

pub const CSRF_STATE_KEY: &str = "auth.csrf-state";
pub const NEXT_URL_KEY: &str = "auth.next-url";
pub const PKCE_VERIFIER_KEY: &str = "auth.pkce-verifier";

#[derive(Debug, Clone, Deserialize)]
struct NextUrl {
    next: Option<String>,
}

async fn oauth_handler(
    auth_session: AuthSession,
    session: Session,
    Form(NextUrl { next }): Form<NextUrl>,
) -> impl IntoResponse {
    let (auth_url, csrf_state, pkce_code_verisfier) = auth_session.backend.authorize_url();

    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .await
        .expect("Serialization should not fail.");

    session
        .insert(NEXT_URL_KEY, next)
        .await
        .expect("Serialization should not fail.");

    session
        .insert(PKCE_VERIFIER_KEY, pkce_code_verisfier)
        .await
        .expect("Serialization should not fail.");

    Redirect::to(auth_url.as_str()).into_response()
}
#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn oauth_callback_handler(
    mut auth_session: AuthSession,
    session: Session,
    Query(AuthzResp {
        code,
        state: new_state,
    }): Query<AuthzResp>,
) -> crate::Result<Response> {
    let Ok(Some(old_state)) = session.get(CSRF_STATE_KEY).await else {
        return Err(eyre!("Session did not contain old csrf state").into());
    };

    let creds = Credentials::OAuth(OAuthCredentials {
        code,
        old_state,
        new_state,
    });

    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(eyre!("Could not authenticate user").into()),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(eyre!("Could not login user").into());
    }

    if let Ok(Some(next)) = session.remove::<String>(NEXT_URL_KEY).await {
        Ok(Redirect::to(&next).into_response())
    } else {
        Ok(Redirect::to("/").into_response())
    }
}

#[tracing::instrument(level = "debug", skip_all)]
async fn logout_handler(mut auth_session: AuthSession) -> crate::Result<Response> {
    auth_session
        .logout()
        .await
        .map_err(|e| eyre!("Failed to logout: {}", e))?;

    Ok(Redirect::to("/login").into_response())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/oauth", post(oauth_handler))
        .route("/oauth_callback", get(oauth_callback_handler))
        .route("/logout", get(logout_handler))
}
use crate::auth::models::{AuthSession, Credentials, RegisterUserRequest};
use crate::auth::services::register;
use crate::err::AppError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use color_eyre::eyre::eyre;
use sqlx::PgPool;

use crate::startup::AppState;

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
    Form(creds): Form<Credentials>,
) -> crate::Result<()> {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(eyre!("Could not authenticate user").into()),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(eyre!("Could not login user").into());
    }

    if let Credentials::Password(pw_creds) = creds {
        //if let Some(ref next) = pw_creds.next {
        //    return Redirect::to(next).into_response();
        //}
    }
    Ok(())
}

#[tracing::instrument(level = "debug", skip_all)]
async fn login_callback_handler(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Credentials::Password(pw_creds) = creds {
        if let Some(ref next) = pw_creds.next {
            return Redirect::to(next).into_response();
        }
    }
    Redirect::to("/").into_response()
}

#[tracing::instrument(level = "debug", skip_all)]
async fn logout_handler(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Credentials::Password(pw_creds) = creds {
        if let Some(ref next) = pw_creds.next {
            return Redirect::to(next).into_response();
        }
    }
    Redirect::to("/").into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/login_callback", get(login_callback_handler))
        .route("/logout", get(logout_handler))
}

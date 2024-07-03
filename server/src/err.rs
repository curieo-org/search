use crate::{auth::AuthError, cache::CacheError, search::SearchError, users::UserError};
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::error::DatabaseError;
use std::fmt::Debug;
use std::{borrow::Cow, collections::HashMap};
use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    SearchError(#[from] SearchError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    UserError(#[from] UserError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    GenericError(#[from] color_eyre::eyre::Error),

    #[error(transparent)]
    Cache(#[from] CacheError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

impl AppError {
    fn to_error_code(&self) -> String {
        match self {
            AppError::SearchError(SearchError::Sqlx(err))
            | AppError::AuthError(AuthError::Sqlx(err))
            | AppError::UserError(UserError::Sqlx(err))
            | AppError::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => "resource_not_found".to_string(),
                sqlx::Error::Protocol(_) => "invalid_data".to_string(),
                sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
                    Some("23505") => "unique_key_violation".to_string(),
                    Some("23503") => "foreign_key_violation".to_string(),
                    _ => "internal_server_error".to_string(),
                },
                _ => "internal_server_error".to_string(),
            },

            AppError::SearchError(err) => match err {
                SearchError::ToxicQuery(_) => format!("toxic_query"),
                SearchError::InvalidData(_) => format!("invalid_data"),
                SearchError::NoResults(_) | SearchError::NoSources(_) => format!("no_results"),
                _ => format!("internal_server_error"),
            },
            AppError::UserError(err) => match err {
                UserError::InvalidData(_) => "invalid_data".to_string(),
                UserError::InvalidPassword(_) => "invalid_password".to_string(),
                _ => "internal_server_error".to_string(),
            },
            AppError::AuthError(err) => match err {
                AuthError::Unauthorized(_) | AuthError::OAuth2(_) => "unauthorized".to_string(),
                AuthError::InvalidSession(_) => "invalid_session".to_string(),
                AuthError::NotWhitelisted(_) => "not_whitelisted".to_string(),
                AuthError::UserAlreadyExists(_) => "user_already_exists".to_string(),
                AuthError::InvalidData(_) => "invalid_data".to_string(),
                _ => "internal_server_error".to_string(),
            },
            _ => "internal_server_error".to_string(),
        }
    }

    fn to_status_code(&self) -> StatusCode {
        match self {
            AppError::SearchError(SearchError::Sqlx(err))
            | AppError::AuthError(AuthError::Sqlx(err))
            | AppError::UserError(UserError::Sqlx(err))
            | AppError::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::Protocol(_) => StatusCode::BAD_REQUEST,
                sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
                    Some("23505") => StatusCode::CONFLICT,
                    Some("23503") => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },

            AppError::SearchError(err) => match err {
                SearchError::ToxicQuery(_) | SearchError::InvalidData(_) => {
                    StatusCode::UNPROCESSABLE_ENTITY
                }
                SearchError::NoResults(_) | SearchError::NoSources(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::AuthError(err) => match err {
                AuthError::Unauthorized(_)
                | AuthError::InvalidSession(_)
                | AuthError::OAuth2(_) => StatusCode::UNAUTHORIZED,
                AuthError::NotWhitelisted(_) => StatusCode::FORBIDDEN,
                AuthError::UserAlreadyExists(_) => StatusCode::CONFLICT,
                AuthError::InvalidData(_) => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::UserError(err) => match err {
                UserError::InvalidData(_) => StatusCode::UNPROCESSABLE_ENTITY,
                UserError::InvalidPassword(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(inner: sqlx::migrate::MigrateError) -> Self {
        AppError::Sqlx(inner.into())
    }
}

#[derive(serde::Serialize, Debug)]
pub struct ErrorMap {
    errors: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl<K, V, I> From<I> for ErrorMap
where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
    I: IntoIterator<Item = (K, V)>,
{
    fn from(i: I) -> Self {
        let mut errors = HashMap::new();

        for (key, val) in i {
            errors.entry(key.into()).or_insert_with(|| val.into());
        }

        ErrorMap { errors }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_code, error_message) = (
            self.to_status_code(),
            self.to_error_code(),
            self.to_string(),
        );

        let error_body = Json(ErrorMap::from([
            ("message", error_message),
            ("error_code", error_code),
        ]));

        // Return a http status code and json body with error message.
        match status_code {
            StatusCode::UNAUTHORIZED => {
                (
                    status_code,
                    // Include the `WWW-Authenticate` challenge required in the specification
                    // for the `401 Unauthorized` response code:
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                    HeaderMap::from_iter([
                        (WWW_AUTHENTICATE, HeaderValue::from_static("Basic")),
                        (WWW_AUTHENTICATE, HeaderValue::from_static("Bearer")),
                    ]),
                    error_body,
                )
                    .into_response()
            }
            _ => (status_code, error_body).into_response(),
        }
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&pool)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
///
/// Something like this would ideally live in a `sqlx-axum` crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `Error` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError>;

    fn catch_constraints(
        self,
        names: &[&str],
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError> {
        self.map_err(|e| match e.into() {
            AppError::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }

    fn catch_constraints(
        self,
        names: &[&str],
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError> {
        self.map_err(|e| match e.into() {
            AppError::Sqlx(sqlx::Error::Database(dbe)) => {
                // TODO: Needs some extra logic to handle multiple constraints.
                if names.contains(&dbe.constraint().unwrap_or_default()) {
                    map_err(dbe)
                } else {
                    AppError::Sqlx(sqlx::Error::Database(dbe))
                }
            }
            e => e,
        })
    }
}

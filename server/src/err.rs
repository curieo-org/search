use crate::{auth::AuthError, cache::CacheError, search::SearchError, users::UserError};
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::fmt::{Debug, Display};
use std::{borrow::Cow, collections::HashMap};
use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    SearchError(SearchError),
    AuthError(AuthError),
    UserError(UserError),
    Sqlx(sqlx::Error),
    GenericError(color_eyre::eyre::Error),
    Cache(CacheError),
    Reqwest(reqwest::Error),
    TaskJoin(#[from] task::JoinError),
}

impl AppError {
    fn to_error_code(&self) -> String {
        match self {
            AppError::SearchError(err) => match err {
                SearchError::ToxicQuery(_) => format!("toxic_query"),
                SearchError::InvalidQuery(_) => format!("invalid_query"),
                SearchError::NoResults(_) | SearchError::NoSources(_) => format!("no_results"),
                _ => format!("internal_server_error"),
            },
            AppError::UserError(err) => match err {
                UserError::NotWhitelisted(_) => "not_whitelisted".to_string(),
                UserError::InvalidData(_) => "invalid_data".to_string(),
                UserError::InvalidPassword(_) => "invalid_password".to_string(),
            },
            AppError::AuthError(err) => match err {
                AuthError::Unauthorized(_) => "unauthorized".to_string(),
                AuthError::InvalidSession(_) => "invalid_session".to_string(),
                AuthError::BackendError(_) => "backend_error".to_string(),
            },

            AppError::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => "resource_not_found".to_string(),
                sqlx::Error::Protocol(_) => "invalid_data".to_string(),
                sqlx::Error::ColumnDecode {
                    index: _,
                    source: _,
                } => "internal_server_error".to_string(),
                sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
                    Some("23505") => "unique_key_violation".to_string(),
                    Some("23503") => "foreign_key_violation".to_string(),
                    _ => "database_error".to_string(),
                },
                _ => "database_error".to_string(),
            },
            _ => "internal_server_error".to_string(),
        }
    }

    fn to_status_code(&self) -> StatusCode {
        match self {
            AppError::SearchError(err) => match err {
                SearchError::ToxicQuery(_) | SearchError::InvalidQuery(_) => {
                    StatusCode::UNPROCESSABLE_ENTITY
                }
                SearchError::NoResults(_) | SearchError::NoSources(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::AuthError(err) => match err {
                AuthError::Unauthorized(_) | AuthError::InvalidSession(_) => {
                    StatusCode::UNAUTHORIZED
                }
                AuthError::BackendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::UserError(err) => match err {
                UserError::NotWhitelisted(_) => StatusCode::FORBIDDEN,
                UserError::InvalidData(_) => StatusCode::UNPROCESSABLE_ENTITY,
                UserError::InvalidPassword(_) => StatusCode::UNAUTHORIZED,
            },

            AppError::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::Protocol(_) => StatusCode::BAD_REQUEST,
                sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
                    Some("23505") => StatusCode::CONFLICT,
                    Some("23503") => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SearchError> for AppError {
    fn from(inner: SearchError) -> Self {
        AppError::SearchError(inner)
    }
}
impl From<AuthError> for AppError {
    fn from(inner: AuthError) -> Self {
        AppError::AuthError(inner)
    }
}
impl From<UserError> for AppError {
    fn from(inner: UserError) -> Self {
        AppError::UserError(inner)
    }
}

impl From<color_eyre::eyre::Error> for AppError {
    fn from(inner: color_eyre::eyre::Error) -> Self {
        AppError::GenericError(inner)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(inner: reqwest::Error) -> Self {
        AppError::Reqwest(inner)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        AppError::Sqlx(inner)
    }
}
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(inner: sqlx::migrate::MigrateError) -> Self {
        AppError::Sqlx(inner.into())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::SearchError(e) => f.write_fmt(format_args!("{:?}", e)),
            AppError::AuthError(e) => e.fmt(f),
            AppError::UserError(e) => e.fmt(f),
            AppError::Sqlx(e) => write!(f, "{}", e),
            AppError::GenericError(e) => write!(f, "{}", e),
            AppError::Cache(e) => write!(f, "{}", e),
            AppError::Reqwest(e) => write!(f, "{}", e),
            AppError::TaskJoin(e) => write!(f, "{}", e),
        }
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

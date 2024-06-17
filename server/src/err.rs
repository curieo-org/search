use crate::auth::BackendError;
use crate::cache::CacheError;
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use color_eyre::eyre::eyre;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    GenericError(color_eyre::eyre::Error),
    Cache(CacheError),
    BadRequest(String),
    Unauthorized,
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    UnprocessableEntity(ErrorMap),
    InternalServerError(String),
}

impl AppError {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    pub fn unprocessable_entity<E>(errors: E) -> Self
    where
        ErrorMap: From<E>,
    {
        Self::UnprocessableEntity(ErrorMap::from(errors))
    }
}

impl From<color_eyre::eyre::Error> for AppError {
    fn from(inner: color_eyre::eyre::Error) -> Self {
        AppError::GenericError(inner)
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

impl From<BackendError> for AppError {
    fn from(e: BackendError) -> Self {
        match e {
            BackendError::Sqlx(e) => AppError::Sqlx(e),
            BackendError::Reqwest(e) => AppError::GenericError(eyre!(e)),
            BackendError::OAuth2(e) => AppError::GenericError(eyre!(e)),
            BackendError::TaskJoin(e) => AppError::GenericError(eyre!(e)),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::GenericError(e) => write!(f, "{}", e),
            AppError::Sqlx(e) => write!(f, "{}", e),
            AppError::Cache(e) => write!(f, "{}", e),
            AppError::BadRequest(e) => write!(f, "{}", e),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden(e) => write!(f, "{}", e),
            AppError::NotFound(e) => write!(f, "{}", e),
            AppError::Conflict(e) => write!(f, "{}", e),
            AppError::UnprocessableEntity(e) => write!(f, "{:?}", e),
            AppError::InternalServerError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct ErrorMap {
    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
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
            errors
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        ErrorMap { errors }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "The request requires user authentication".to_string(),
            ),
            AppError::UnprocessableEntity(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Unprocessable entity: {:?}", e),
            ),
            AppError::GenericError(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Generic error: {}", e),
            ),
            AppError::Cache(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cache error: {}", e),
            ),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
            AppError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::Sqlx(sqlx::Error::RowNotFound) => (
                StatusCode::NOT_FOUND,
                "The requested resource was not found".to_string(),
            ),
            AppError::Sqlx(sqlx::Error::Protocol(msg)) => (
                StatusCode::BAD_REQUEST,
                format!("The provided data is invalid: {}", msg),
            ),
            AppError::Sqlx(sqlx::Error::ColumnDecode { index, ref source }) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("SQLx column decode error: {} at index {}", source, index),
            ),
            AppError::Sqlx(sqlx::Error::Database(db_err)) => match db_err.code().as_deref() {
                Some("23505") => (
                    StatusCode::CONFLICT,
                    format!("Unique constraint violation: {}", db_err.to_string()),
                ),
                Some("23503") => (
                    StatusCode::BAD_REQUEST,
                    format!("Foreign key constraint violation: {}", db_err.to_string()),
                ),
                _ => (StatusCode::BAD_REQUEST, db_err.to_string()),
            },
            AppError::Sqlx(sqlx::Error::Io(io_err)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, io_err.to_string())
            }
            AppError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_body = Json(ErrorMap::from([("message", error_message)]));

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

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
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    UnprocessableEntity(ErrorMap),
    Sqlx(sqlx::Error),
    GenericError(color_eyre::eyre::Error),
    Cache(CacheError),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Cache(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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
            AppError::UnprocessableEntity(e) => write!(f, "{:?}", e),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Cache(e) => write!(f, "{}", e),
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
        match self {
            AppError::Unauthorized => {
                return (
                    self.status_code(),
                    // Include the `WWW-Authenticate` challenge required in the specification
                    // for the `401 Unauthorized` response code:
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                    HeaderMap::from_iter([
                        (WWW_AUTHENTICATE, HeaderValue::from_static("Basic")),
                        (WWW_AUTHENTICATE, HeaderValue::from_static("Bearer")),
                    ]),
                    Json(ErrorMap::from([("message", "Unauthorized")])),
                )
                    .into_response();
            }
            AppError::UnprocessableEntity(e) => {
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into_response();
            }
            AppError::Sqlx(ref e) => error!("SQLx error: {:?}", e),
            AppError::GenericError(ref e) => error!("Generic error: {:?}", e),
            AppError::Cache(ref e) => error!("Redis error: {:?}", e),
        };

        // Return a http status code and json body with error message.
        (
            self.status_code(),
            Json(ErrorMap::from([("message", "Something went wrong")])),
        )
            .into_response()
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

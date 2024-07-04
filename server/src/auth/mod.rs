use ::oauth2::ConfigurationError;
pub use api_models::*;
pub use models::*;
use oauth2::basic::BasicRequestTokenError;
use oauth2::reqwest::AsyncHttpClientError;
use openidconnect::{DiscoveryError, UserInfoError};
pub use routes::*;
pub use services::*;
use tokio::task;
pub mod api_models;
pub mod models;
pub mod oauth_2;
pub mod routes;
pub mod services;
pub(crate) mod sessions;
pub mod utils;

#[derive(Debug, thiserror::Error)]
pub enum OIDCError {
    #[error(transparent)]
    Discovery(#[from] DiscoveryError<AsyncHttpClientError>),

    #[error(transparent)]
    Configuration(#[from] ConfigurationError),

    #[error(transparent)]
    UserInfo(#[from] UserInfoError<AsyncHttpClientError>),

    ///
    /// An unexpected error occurred.
    ///
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    OAuth2(#[from] BasicRequestTokenError<AsyncHttpClientError>),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),

    #[error(transparent)]
    OIDC(#[from] OIDCError),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Invalid session: {0}")]
    InvalidSession(String),
    #[error("Other error: {0}")]
    UserAlreadyExists(String),
    #[error("Other error: {0}")]
    InvalidData(String),
    #[error("Not whitelisted: {0}")]
    NotWhitelisted(String),
    #[error("Other error: {0}")]
    Other(String),
}

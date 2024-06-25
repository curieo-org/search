use ::oauth2::ConfigurationError;
pub use models::*;
use oauth2::basic::BasicRequestTokenError;
use oauth2::reqwest::AsyncHttpClientError;
use openidconnect::{DiscoveryError, UserInfoError};
pub use routes::*;
pub use services::*;
use tokio::task;

pub mod models;
pub mod oauth_2;
pub mod routes;
pub mod services;
pub(crate) mod sessions;
mod utils;

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
}

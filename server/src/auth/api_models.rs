use crate::secrets::Secret;
use oauth2::{basic::BasicRequestTokenError, reqwest::AsyncHttpClientError};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use tokio::task;
use validator::Validate;

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

#[derive(Deserialize, Clone, Debug, Validate)]
#[serde(remote = "Self")]
pub struct RegisterUserRequest {
    #[validate(email)]
    pub email: String,
    pub password: Option<Secret<String>>,
    pub access_token: Option<Secret<String>>,
}

impl<'de> Deserialize<'de> for RegisterUserRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Self::deserialize(deserializer)?;
        if s.password.is_some() && s.access_token.is_some() {
            return Err(D::Error::custom(
                "should only have password or access token",
            ));
        }

        Ok(s)
    }
}

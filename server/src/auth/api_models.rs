use crate::err::ErrorExt;
use crate::secrets::Secret;
use axum::http::StatusCode;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized(String),
    InvalidSession(String),
    BackendError(String),
}

impl ErrorExt for AuthError {
    fn to_error_code(&self) -> String {
        match self {
            AuthError::Unauthorized(_) => "unauthorized".to_string(),
            AuthError::InvalidSession(_) => "invalid_session".to_string(),
            AuthError::BackendError(_) => "backend_error".to_string(),
        }
    }

    fn to_status_code(&self) -> StatusCode {
        match self {
            AuthError::Unauthorized(_) | AuthError::InvalidSession(_) => StatusCode::UNAUTHORIZED,
            AuthError::BackendError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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

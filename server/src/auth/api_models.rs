use crate::secrets::Secret;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized(String),
    InvalidSession(String),
    BackendError(String),
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

use crate::secrets::Secret;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePasswordRequest {
    pub old_password: Secret<String>,
    pub new_password: Secret<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 50))]
    pub fullname: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub company: Option<String>,
}

impl UpdateProfileRequest {
    pub fn has_any_value(&self) -> bool {
        [
            self.fullname.is_some(),
            self.title.is_some(),
            self.company.is_some(),
        ]
        .iter()
        .any(|&x| x)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Other error: {0}")]
    Other(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

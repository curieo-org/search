use crate::secrets::Secret;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::types::time;
use std::fmt::Debug;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UserOut {
    pub user_id: uuid::Uuid,
    pub username: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: Secret<Option<String>>,
    pub access_token: Secret<Option<String>>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl AuthUser for User {
    type Id = uuid::Uuid;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        if let Some(access_token) = &self.access_token.expose() {
            return access_token.as_bytes();
        }

        if let Some(password) = &self.password_hash.expose() {
            return password.as_bytes();
        }

        &[]
    }
}

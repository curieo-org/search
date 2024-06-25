use crate::auth::AuthSession;
use crate::secrets::Secret;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::types::time;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum UserGroup {
  Alpha,
  Beta,
  Public,
} // Move Public to top before public release

impl From<i32> for UserGroup {
  fn from(value: i32) -> Self {
      match value {
        0 => UserGroup::Alpha,
        1 => UserGroup::Beta,
        _ => UserGroup::Public,
      }
  }
}

#[derive(sqlx::FromRow, Serialize, Clone, Debug)]
pub struct UserRecord {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub fullname: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
    pub user_group: UserGroup,
}

impl From<User> for UserRecord {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            fullname: user.fullname,
            title: user.title,
            company: user.company,
            user_group: user.user_group,
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub fullname: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
    pub user_group: UserGroup,
    pub password_hash: Secret<Option<String>>,
    pub access_token: Secret<Option<String>>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/signin").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_session = AuthSession::from_request_parts(parts, state).await?;
        auth_session
            .user
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePasswordRequest {
    pub old_password: Secret<String>,
    pub new_password: Secret<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub fullname: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
}


impl UpdateProfileRequest {
    pub fn has_any_value(&self) -> bool {
        [self.username.is_some(),
         self.email.is_some(),
         self.fullname.is_some(),
         self.title.is_some(),
         self.company.is_some()
        ].iter().any(|&x| x)
    }
}
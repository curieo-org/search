use std::fmt::Debug;

use async_trait::async_trait;
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{AuthUser, AuthnBackend, UserId};
use oauth2::{
    basic::{BasicClient, BasicRequestTokenError},
    reqwest::{async_http_client, AsyncHttpClientError},
    url::Url,
    AuthorizationCode, CsrfToken, TokenResponse,
};
use password_auth::verify_password;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::types::time;
use sqlx::{FromRow, PgPool};
use tokio::task;

use crate::secrets::Secret;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct UserOut {
    pub user_id: uuid::Uuid,
    pub username: String,
}

#[derive(sqlx::FromRow, Clone, Deserialize, Serialize, Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: Secret<Option<String>>,
    pub access_token: Secret<Option<String>>,

    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(remote = "Self")]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password_hash: Secret<Option<String>>,
    pub access_token: Secret<Option<String>>,
}

impl<'de> Deserialize<'de> for CreateUserRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Self::deserialize(deserializer)?;
        if s.password_hash.expose().is_some() && s.access_token.expose().is_some() {
            return Err(serde::de::Error::custom(
                "should only have password or access token",
            ));
        }

        Ok(s)
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

#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
    Password(PasswordCreds),
    OAuth(OAuthCreds),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordCreds {
    pub username: String,
    pub password: Secret<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCreds {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    login: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(sqlx::Error),

    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    OAuth2(BasicRequestTokenError<AsyncHttpClientError>),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Debug, Clone)]
pub struct PostgresBackend {
    db: PgPool,
    client: BasicClient,
}

impl PostgresBackend {
    pub fn new(db: PgPool, client: BasicClient) -> Self {
        Self { db, client }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client.authorize_url(CsrfToken::new_random).url()
    }
}

#[allow(clippy::blocks_in_conditions)]
#[async_trait]
impl AuthnBackend for PostgresBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    #[tracing::instrument(level = "debug", skip(self), ret, err)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Password(password_cred) => {
                let user: Option<Self::User> = sqlx::query_as(
                    "select * from users where username = ? and password is not null",
                )
                .bind(password_cred.username)
                .fetch_optional(&self.db)
                .await
                .map_err(Self::Error::Sqlx)?;

                // Verifying the password is blocking and potentially slow, so we have to do it via
                // `spawn_blocking`.
                task::spawn_blocking(move || {
                    // password-based authentication. Compare our form  input with an argon2
                    // password hash.
                    // To prevent timed side-channel attacks, so we always compare the password
                    // hash, even if the user doesn't exist.
                    return match user {
                        // If there is no user with this username we dummy verify the password.
                        None => dummy_verify_password(password_cred.password.expose()),
                        Some(user) => {
                            // If the user exists, but has no password, we dummy verify the password.
                            let Some(password) = user.password_hash.expose() else {
                                return dummy_verify_password(password_cred.password.expose());
                            };

                            // If the user exists and has a password, we verify the password.
                            match verify_password(
                                password_cred.password.expose(),
                                password.as_ref(),
                            ) {
                                Ok(_) => Ok(Some(user)),
                                _ => Ok(None),
                            }
                        }
                    };
                })
                .await?
            }
            Credentials::OAuth(oauth_creds) => {
                // Ensure the CSRF state has not been tampered with.
                if oauth_creds.old_state.secret() != oauth_creds.new_state.secret() {
                    return Ok(None);
                };

                // Process authorization code, expecting a token response back.
                let token_res = self
                    .client
                    .exchange_code(AuthorizationCode::new(oauth_creds.code))
                    .request_async(async_http_client)
                    .await
                    .map_err(Self::Error::OAuth2)?;

                // Use access token to request user info.
                let user_info = reqwest::Client::new()
                    .get("https://api.github.com/user")
                    .header(USER_AGENT.as_str(), "axum-login") // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
                    .header(
                        AUTHORIZATION.as_str(),
                        format!("Bearer {}", token_res.access_token().secret()),
                    )
                    .send()
                    .await
                    .map_err(Self::Error::Reqwest)?
                    .json::<UserInfo>()
                    .await
                    .map_err(Self::Error::Reqwest)?;

                // Persist user in our database, so we can use `get_user`.
                let user = sqlx::query_as(
                    r#"
                    insert into users (username, access_token)
                    values (?, ?)
                    on conflict(username) do update
                    set access_token = excluded.access_token
                    returning *
                    "#,
                )
                .bind(user_info.login)
                .bind(token_res.access_token().secret())
                .fetch_one(&self.db)
                .await
                .map_err(Self::Error::Sqlx)?;

                Ok(Some(user))
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self), ret, err)]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        sqlx::query_as("select * from users where id = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(Self::Error::Sqlx)
    }
}

// This is a dummy function to prevent timing attacks.
fn dummy_verify_password(pw: &str) -> Result<Option<User>, BackendError> {
    let _ = verify_password(
        pw,
        "smop-flurp-fjoing-Idoubtsomeonewouldeverusethispassword!",
    );
    // Even if the password is correct we still return `Ok(None)`.
    Ok(None)
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<PostgresBackend>;

use crate::auth::oauth2::OAuth2Client;
use crate::auth::utils;
use crate::err::AppError;
use crate::err::ErrorExt;
use crate::secrets::Secret;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::users::User;
use async_trait::async_trait;
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum::http::StatusCode;
use axum_login::{AuthnBackend, UserId};
use oauth2::url::Url;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier, TokenResponse};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;

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

#[derive(Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: Option<Secret<String>>,
    pub access_token: Option<Secret<String>>,
}

impl<'de> Deserialize<'de> for RegisterUserRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Self::deserialize(deserializer)?;
        if s.password.is_some() && s.access_token.is_some() {
            return Err(Error::custom("should only have password or access token"));
        }

        Ok(s)
    }
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    login: String,
}

#[derive(Debug, Clone)]
pub struct PostgresBackend {
    db: PgPool,
    oauth2_clients: Vec<OAuth2Client>,
}

impl PostgresBackend {
    pub fn new(db: PgPool, oauth2_clients: Vec<OAuth2Client>) -> Self {
        Self { db, oauth2_clients }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        self.oauth2_clients
            .first()
            .unwrap()
            .authorize_url(CsrfToken::new_random)
    }
}

#[tracing::instrument(level = "info", ret, err)]
async fn password_authenticate(
    db: &PgPool,
    password_credentials: PasswordCredentials,
) -> crate::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "select * from users where username = $1 and password_hash is not null",
        password_credentials.username
    )
    .fetch_optional(db)
    .await?;

    // Verifying the password is blocking and potentially slow, so we do it via
    // `spawn_blocking`.
    spawn_blocking_with_tracing(move || {
        utils::verify_user_password(user, password_credentials.password)
    })
    .await?
}

#[tracing::instrument(level = "info", ret, err)]
async fn oauth_authenticate(
    db: &PgPool,
    oauth2_clients: &[OAuth2Client],
    oauth_creds: OAuthCredentials,
) -> crate::Result<Option<User>> {
    // Ensure the CSRF state has not been tampered with.
    if oauth_creds.old_state.secret() != oauth_creds.new_state.secret() {
        return Ok(None);
    };

    let client = oauth2_clients.first().unwrap();
    // Process authorization code, expecting a token response back.
    let token_res = client
        .exchange_code(AuthorizationCode::new(oauth_creds.code))
        .await
        .map_err(|e| AuthError::Unauthorized(e.to_string()))?;

    // Use access token to request user info.
    let user_info = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header(USER_AGENT.as_str(), "axum-login") // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
        .header(
            AUTHORIZATION.as_str(),
            format!("Bearer {}", token_res.access_token().secret()),
        )
        .send()
        .await?
        .json::<UserInfo>()
        .await?;

    // Persist user in our database, so we can use `get_user`.
    let user = sqlx::query_as!(
        User,
        "
        insert into users (username, access_token)
        values ($1, $2)
        on conflict(username) do update
        set access_token = excluded.access_token
        returning *
        ",
        user_info.login,
        token_res.access_token().secret()
    )
    .fetch_one(db)
    .await?;

    Ok(Some(user))
}

#[allow(clippy::blocks_in_conditions)]
#[async_trait]
impl AuthnBackend for PostgresBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AppError;

    #[tracing::instrument(level = "info", skip(self), ret, err)]
    async fn authenticate(&self, creds: Self::Credentials) -> crate::Result<Option<Self::User>> {
        match creds {
            Credentials::Password(password_cred) => {
                password_authenticate(&self.db, password_cred).await
            }
            Credentials::OAuth(oauth_creds) => {
                oauth_authenticate(&self.db, &self.oauth2_clients, oauth_creds).await
            }
        }
    }

    #[tracing::instrument(level = "info", skip(self), ret, err)]
    async fn get_user(&self, user_id: &UserId<Self>) -> crate::Result<Option<Self::User>> {
        sqlx::query_as!(
            Self::User,
            "select * from users where user_id = $1",
            user_id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Credentials {
    Password(PasswordCredentials),
    OAuth(OAuthCredentials),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordCredentials {
    pub username: String,
    pub password: Secret<String>,
    pub csrf_token: Option<CsrfToken>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCredentials {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<PostgresBackend>;

#[cfg(test)]
mod tests {
    use crate::auth::utils::dummy_verify_password;
    use crate::secrets::Secret;

    #[tokio::test]
    async fn test_dummy_verify_password() {
        assert!(dummy_verify_password(Secret::new("password")).is_ok());
    }
}

pub struct WhitelistedEmail {
    pub email: String,
    pub approved: bool,
}

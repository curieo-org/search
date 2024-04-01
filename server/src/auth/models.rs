use crate::auth::utils;
use crate::secrets::Secret;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::users::User;
use async_trait::async_trait;
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{AuthnBackend, UserId};
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use oauth2::reqwest::{async_http_client, AsyncHttpClientError};
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RevocationUrl, TokenResponse, TokenUrl,
};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::PgPool;
use tokio::task;
use tracing::debug;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password_hash: Option<Secret<String>>,
    pub access_token: Option<Secret<String>>,
}

impl Serialize for RegisterUserRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for RegisterUserRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Self::deserialize(deserializer)?;
        if s.password_hash.is_some() && s.access_token.is_some() {
            return Err(Error::custom("should only have password or access token"));
        }

        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub struct PostgresBackend {
    db: PgPool,
    oauth_clients: OAuth2Clients,
}

#[derive(Debug, Clone)]
pub struct OAuth2Clients {
    pub github: GitHubOAuthClient,
    pub google: GoogleOAuthClient,
}

impl Default for OAuth2Clients {
    fn default() -> Self {
        Self {
            github: GitHubOAuthClient,
            google: GoogleOAuthClient,
        }
    }
}

#[async_trait]
trait OAuth2Client {
    const CLIENT_ID: &'static str;
    const CLIENT_SECRET: &'static str;
    const AUTH_URL: &'static str;
    const TOKEN_URL: &'static str;
    const REVOCATION_URL: &'static str;

    fn client(&self) -> BasicClient {
        BasicClient::new(
            ClientId::new(Self::CLIENT_ID.to_string()),
            Some(ClientSecret::new(Self::CLIENT_SECRET.to_string())),
            AuthUrl::new(Self::AUTH_URL.to_string()).expect("invalid auth url"),
            Some(TokenUrl::new(Self::TOKEN_URL.to_string()).expect("invalid token url")),
        )
        .set_revocation_uri(
            RevocationUrl::new(Self::REVOCATION_URL.to_string()).expect("invalid auth url"),
        )
    }
    fn authorize_url<S>(&self, state: S) -> (Url, CsrfToken, PkceCodeVerifier)
    where
        S: FnOnce() -> CsrfToken,
    {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (url, token) = self
            .client()
            .authorize_url(state)
            //.add_scope(Scope::new("user:email".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        (url, token, pkce_code_verifier)
    }
    async fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, BasicRequestTokenError<AsyncHttpClientError>> {
        self.client()
            .exchange_code(code)
            .request_async(async_http_client)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GitHubOAuthClient;

impl OAuth2Client for GitHubOAuthClient {
    const CLIENT_ID: &'static str = "github_client_id";
    const CLIENT_SECRET: &'static str = "github_client_secret";
    const AUTH_URL: &'static str = "";
    const TOKEN_URL: &'static str = "";
    const REVOCATION_URL: &'static str = "";
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthClient;

impl OAuth2Client for GoogleOAuthClient {
    const CLIENT_ID: &'static str = "";
    const CLIENT_SECRET: &'static str = "";
    const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/v2/auth";
    const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v3/token";
    const REVOCATION_URL: &'static str = "https://oauth2.googleapis.com/revoke";
}

impl PostgresBackend {
    pub fn new(db: PgPool, oauth_clients: OAuth2Clients) -> Self {
        Self { db, oauth_clients }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        self.oauth_clients
            .google
            .authorize_url(CsrfToken::new_random)
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
                let user = sqlx::query_as!(
                    User,
                    "select * from users where username = $1 and password_hash is not null",
                    password_cred.username
                )
                .fetch_optional(&self.db)
                .await
                .map_err(Self::Error::Sqlx)?;

                debug!("user: {:?}", user);

                // Verifying the password is blocking and potentially slow, so we do it via
                // `spawn_blocking`.
                spawn_blocking_with_tracing(move || {
                    utils::verify_user_password(user, password_cred.password)
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
                    .oauth_clients
                    .github
                    .exchange_code(AuthorizationCode::new(oauth_creds.code))
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
        sqlx::query_as!(
            Self::User,
            "select * from users where user_id = $1",
            user_id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(Self::Error::Sqlx)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Credentials {
    Password(PasswordCreds),
    OAuth(OAuthCreds),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordCreds {
    pub username: String,
    pub password: Secret<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuthCreds {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
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

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<PostgresBackend>;

#[cfg(test)]
mod tests {
    use crate::auth::models::{Credentials, OAuth2Clients, PasswordCreds};
    use crate::auth::utils::dummy_verify_password;
    use crate::secrets::Secret;

    #[tokio::test]
    async fn test_dummy_verify_password() {
        assert!(dummy_verify_password(Secret::new("password")).is_ok());
    }

    #[tokio::test]
    async fn test_oauth2_clients() {
        let clients = OAuth2Clients::default();
        //let _ = clients.github.client();
        //let _ = clients.google.client();
    }

    #[test]
    fn test_creds() {
        let creds = Credentials::Password(PasswordCreds {
            username: "test".to_string(),
            password: Secret::new("password".to_string()),
            next: None,
        });
        println!(
            "as_json: {:?}",
            serde_json::to_string_pretty(&creds).unwrap()
        );

        assert_eq!(1, 2)
    }
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    login: String,
}

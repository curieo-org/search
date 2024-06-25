use crate::auth::oauth_2::{Issuer, OIDCClient};
use crate::auth::{utils, AuthError, OIDCError};
use crate::secrets::Secret;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::users::User;
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use openidconnect::core::CoreUserInfoClaims;
use openidconnect::{AccessToken, CsrfToken};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;

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

#[derive(Debug, Clone)]
pub struct PostgresBackend {
    db: PgPool,
    oidc_clients: Vec<OIDCClient>,
}

impl PostgresBackend {
    pub fn new(db: PgPool, oidc_clients: Vec<OIDCClient>) -> Self {
        Self { db, oidc_clients }
    }

    fn get_oidc_client(&self, issuer: &Issuer) -> Result<OIDCClient, OIDCError> {
        self.oidc_clients
            .iter()
            .find(|c| &c.issuer == issuer)
            .ok_or_else(|| {
                OIDCError::Other(format!(
                    "Invalid configuration. No client exists for issuer `{issuer:?}`."
                ))
            })
            .cloned()
    }

    #[tracing::instrument(level = "debug", ret, err)]
    async fn password_authenticate(
        &self,
        password_credentials: PasswordCredentials,
    ) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as!(
            User,
            "select * from users where username = $1 and password_hash is not null",
            password_credentials.username
        )
        .fetch_optional(&self.db)
        .await
        .map_err(AuthError::Sqlx)?;

        // Verifying the password is blocking and potentially slow, so we do it via
        // `spawn_blocking`.
        spawn_blocking_with_tracing(move || {
            utils::verify_user_password(user, password_credentials.password)
        })
        .await?
    }

    #[tracing::instrument(level = "debug", ret, err)]
    async fn oauth_authenticate(
        &self,
        oauth_creds: OAuthCredentials,
    ) -> Result<Option<User>, AuthError> {
        //// Ensure the CSRF state has not been tampered with.
        if oauth_creds.old_state.secret() != oauth_creds.new_state.secret() {
            return Ok(None);
        };

        let OIDCClient { client, .. } = self.get_oidc_client(&oauth_creds.issuer)?;

        //// Process authorization code, expecting a token response back.
        let token_res = client
            .exchange_code(AuthorizationCode::new(oauth_creds.code))
            .request_async(async_http_client)
            .await?;

        // Use access token to request user info.
        let user_info: CoreUserInfoClaims = client
            .user_info(token_res.access_token().clone(), None)
            .map_err(OIDCError::Configuration)?
            .request_async(async_http_client)
            .await
            .map_err(OIDCError::UserInfo)?;

        let username = extract_username(user_info)?;

        //// Persist user in our database, so we can use `get_user`.
        let user = sqlx::query_as!(
            User,
            r#"
            insert into users (username, access_token)
            values ($1, $2)
            on conflict(username) do update
            set access_token = excluded.access_token
            returning *
            "#,
            username,
            token_res.access_token().secret()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Some(user))
    }

    #[tracing::instrument(level = "debug", ret, err)]
    async fn validate_access_token(
        &self,
        AccessTokenCredentials {
            username,
            issuer,
            access_token,
        }: AccessTokenCredentials,
    ) -> Result<Option<User>, AuthError> {
        let OIDCClient { client, .. } = self.get_oidc_client(&issuer)?;

        // Use access token to request user info.
        let user_info: CoreUserInfoClaims = client
            .user_info(access_token.clone(), None)
            .map_err(OIDCError::Configuration)?
            .request_async(async_http_client)
            .await
            .map_err(OIDCError::UserInfo)?;

        let extracted_username = extract_username(user_info)?;

        if username != extracted_username {
            return Err(
                OIDCError::Other("Username does not match provided username".to_string()).into(),
            );
        }

        //// Persist user in our database, so we can use `get_user`.
        let user = sqlx::query_as!(
            User,
            r#"
            insert into users (username, access_token)
            values ($1, $2)
            on conflict(username) do update
            set access_token = excluded.access_token
            returning *
            "#,
            username,
            access_token.secret()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Some(user))
    }
}

fn extract_username(claims: CoreUserInfoClaims) -> Result<String, AuthError> {
    if let Some(preferred_username) = claims.preferred_username() {
        return Ok(preferred_username.as_str().to_string());
    }
    if let Some(email_verified) = claims.email_verified() {
        if email_verified {
            if let Some(email) = claims.email() {
                return Ok(email.as_str().to_string());
            }
        }
    }
    Err(
        OIDCError::Other("Could not retrieve valid username from user info claims".to_string())
            .into(),
    )
}

#[allow(clippy::blocks_in_conditions)]
#[async_trait]
impl AuthnBackend for PostgresBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    #[tracing::instrument(level = "debug", skip(self), ret, err)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Password(password_cred) => self.password_authenticate(password_cred).await,
            Credentials::OAuth(oauth_creds) => self.oauth_authenticate(oauth_creds).await,
            Credentials::AccessToken(token) => self.validate_access_token(token).await,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Credentials {
    Password(PasswordCredentials),
    OAuth(OAuthCredentials),
    AccessToken(AccessTokenCredentials),
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
    pub issuer: Issuer,
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AccessTokenCredentials {
    pub username: String,
    pub issuer: Issuer,
    pub access_token: AccessToken,
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

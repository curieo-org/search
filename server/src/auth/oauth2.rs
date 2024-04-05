use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use oauth2::reqwest::{async_http_client, AsyncHttpClientError};
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RevocationUrl, TokenUrl,
};
use serde::{Deserialize, Deserializer};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum OAuth2Provider {
    Google,
}

impl<'de> Deserialize<'de> for OAuth2Provider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "google" => Ok(OAuth2Provider::Google),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid OAuth2 provider: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct OAuth2ClientInfo {
    pub provider: OAuth2Provider,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub revocation_url: RevocationUrl,
}

impl From<OAuth2ClientInfo> for BasicClient {
    fn from(info: OAuth2ClientInfo) -> Self {
        BasicClient::new(
            info.client_id,
            Some(info.client_secret),
            info.auth_url,
            Some(info.token_url),
        )
        .set_revocation_uri(info.revocation_url)
    }
}

#[derive(Debug, Clone)]
pub struct OAuth2Client {
    client: Arc<BasicClient>,
    info: OAuth2ClientInfo,
}

impl OAuth2Client {
    pub fn client(&self) -> &BasicClient {
        self.client.as_ref()
    }
    pub fn provider(&self) -> &OAuth2Provider {
        &self.info.provider
    }
    pub fn client_id(&self) -> &ClientId {
        &self.info.client_id
    }
    pub fn client_secret(&self) -> &ClientSecret {
        &self.info.client_secret
    }
    pub fn auth_url(&self) -> &AuthUrl {
        &self.info.auth_url
    }
    pub fn token_url(&self) -> &TokenUrl {
        &self.info.token_url
    }
    pub fn revocation_url(&self) -> &RevocationUrl {
        &self.info.revocation_url
    }

    pub fn authorize_url<S>(&self, state: S) -> (Url, CsrfToken, PkceCodeVerifier)
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
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, BasicRequestTokenError<AsyncHttpClientError>> {
        self.client()
            .exchange_code(code)
            .request_async(async_http_client)
            .await
    }
}

impl<'de> Deserialize<'de> for OAuth2Client {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let client_info = OAuth2ClientInfo::deserialize(deserializer)?;
        let basic_client = BasicClient::from(client_info.clone());

        Ok(OAuth2Client {
            client: Arc::new(basic_client),
            info: client_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_oauth2_client() {
        let json = json!({
            "provider": "google",
            "client_id": "client_id",
            "client_secret": "client_secret",
            "auth_url": "https://auth_url",
            "token_url": "https://token_url",
            "revocation_url": "https://revocation_url"
        });

        let client: OAuth2Client = serde_json::from_value(json).unwrap();
        assert_eq!(client.provider(), &OAuth2Provider::Google);
        assert_eq!(client.client_id().as_str(), "client_id");
        assert_eq!(client.client_secret().secret(), "client_secret");
        assert_eq!(client.auth_url().as_str(), "https://auth_url");
        assert_eq!(client.token_url().as_str(), "https://token_url");
        assert_eq!(client.revocation_url().as_str(), "https://revocation_url");
    }
}

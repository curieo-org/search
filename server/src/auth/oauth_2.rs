use oauth2::{ClientId, ClientSecret};
use openidconnect::core::CoreClient;
use openidconnect::IssuerUrl;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq)]
pub enum Issuer {
    Google,
}

impl<'de> Deserialize<'de> for Issuer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "google" => Ok(Issuer::Google),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid OAuth2 provider: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OIDCClientInfo {
    pub issuer: Issuer,
    pub issuer_url: IssuerUrl,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
}

#[derive(Debug, Clone)]
pub struct OIDCClient {
    pub issuer: Issuer,
    pub client: CoreClient,
}

#[cfg(test)]
mod tests {
    use crate::auth::oauth_2::{Issuer, OIDCClientInfo};
    use serde_json::json;

    #[test]
    fn deserialize_oidc_client_info() {
        let json = json!({
            "issuer": "google",
            "issuer_url": "https://issuer_url",
            "client_id": "client_id",
            "client_secret": "client_secret",
        });

        let client_info: OIDCClientInfo = serde_json::from_value(json).unwrap();
        assert_eq!(client_info.issuer, Issuer::Google);
        assert_eq!(client_info.issuer_url.url().as_str(), "https://issuer_url/");
        assert_eq!(client_info.client_id.as_str(), "client_id");
        assert_eq!(client_info.client_secret.secret(), "client_secret");
    }
}

use std::{env, fmt::Display};

use crate::auth::oauth2::OAuth2Client;
use crate::secrets::Secret;
use config::{Config, Environment, File};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub enum LogFmt {
    Json,
    Pretty,
    Default,
}

impl<'de> Deserialize<'de> for LogFmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "json" => LogFmt::Json,
            "pretty" => LogFmt::Pretty,
            _ => LogFmt::Default,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub level: String,
    pub format: LogFmt,
}

#[derive(Debug, Clone)]
pub enum Env {
    Local,
    Dev,
    Staging,
    Prod,
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Env::Local => "local",
            Env::Dev => "dev",
            Env::Staging => "staging",
            Env::Prod => "prod",
        };

        write!(f, "{}", value)
    }
}

impl<'de> Deserialize<'de> for Env {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "prod" => Env::Prod,
            "staging" => Env::Staging,
            "dev" => Env::Dev,
            _ => Env::Local,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub environment: Env,
    pub log: Log,
    pub host: String,
    pub port: u16,
    pub db: Secret<String>,
    pub cache: Secret<String>,
    pub rag_api: String,
    pub rag_api_username: Secret<String>,
    pub rag_api_password: Secret<String>,
    pub cache_max_sorted_size: i64,
    pub oauth2_clients: Vec<OAuth2Client>,
}

impl Settings {
    pub fn new() -> Self {
        // Inject values from a .env file into environment if it exists
        dotenv().ok();

        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".into());

        let settings = Config::builder()
            .set_default("environment", environment.clone())
            .unwrap()
            // Add our common configuration values that _generally_ apply across all environments
            .add_source(File::with_name("config/default"))
            // Add environment-specific overrides if they exist
            .add_source(File::with_name(&format!("config/{environment}")).required(false))
            // Add local overrides if they exist (this should only be used locally)
            .add_source(File::with_name("config/local").required(false))
            // Allow for overrides via environment variables prefixed with APP_ (i.e., "APP_DEBUG=1")
            .add_source(Environment::default().try_parsing(true).separator("__"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(Settings::new);

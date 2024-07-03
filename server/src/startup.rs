use crate::auth::oauth2::OAuth2Client;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag::brave_search;
use crate::{cache::CachePool, routing::router, settings::Settings};
use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::eyre::eyre;
use log::info;
use regex::Regex;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tonic::transport::Channel;

pub struct Application {
    port: u16,
    server: Serve<IntoMakeService<Router>, Router>,
}

impl Application {
    pub async fn build(settings: Settings) -> crate::Result<Self> {
        let address = format!("{}:{}", settings.host, settings.port);
        info!("Running on {address}");

        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| eyre!("Failed to bind to address: {}", e))?;
        let port = listener
            .local_addr()
            .map_err(|e| eyre!("Failed to get local address: {}", e))?
            .port();
        let server = run(listener, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> crate::Result<()> {
        Ok(self
            .server
            .await
            .map_err(|e| eyre!("Server error: {}", e))?)
    }
}

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub cache: CachePool,
    pub agency_service: AgencyServiceClient<Channel>,
    pub oauth2_clients: Vec<OAuth2Client>,
    pub settings: Settings,
    pub brave_config: brave_search::BraveAPIConfig,
    pub openai_stream_regex: regex::Regex,
}

impl AppState {
    pub async fn new(
        db: PgPool,
        cache: CachePool,
        agency_service: AgencyServiceClient<Channel>,
        oauth2_clients: Vec<OAuth2Client>,
        settings: Settings,
        brave_config: brave_search::BraveAPIConfig,
        openai_stream_regex: regex::Regex,
    ) -> crate::Result<Self> {
        Ok(Self {
            db,
            cache,
            agency_service,
            oauth2_clients,
            settings,
            brave_config,
            openai_stream_regex,
        })
    }

    pub async fn initialize(settings: Settings) -> crate::Result<Self> {
        Ok(Self {
            db: db_connect(settings.db.expose()).await?,
            cache: CachePool::new(&settings.cache).await?,
            agency_service: agency_service_connect(settings.agency_api.expose()).await?,
            oauth2_clients: settings.oauth2_clients.clone(),
            brave_config: brave_search::prepare_brave_api_config(&settings.brave),
            settings,
            openai_stream_regex: Regex::new(r#"\"content\":\"(.*?)\"}"#)
                .map_err(|e| eyre!("Failed to compile OpenAI stream regex: {}", e))?,
        })
    }
}

pub async fn db_connect(database_url: &str) -> crate::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|e| e.into())
}

pub async fn agency_service_connect(
    agency_service_url: &str,
) -> crate::Result<AgencyServiceClient<Channel>> {
    let agency_service = AgencyServiceClient::connect(agency_service_url.to_owned())
        .await
        .map_err(|e| eyre!("Failed to connect to agency service: {}", e))?;

    Ok(agency_service)
}

async fn run(
    listener: TcpListener,
    settings: Settings,
) -> crate::Result<Serve<IntoMakeService<Router>, Router>> {
    let state = AppState::initialize(settings).await?;
    sqlx::migrate!().run(&state.db).await?;

    let app = router(state)?;

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}

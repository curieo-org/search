use crate::auth::oauth_2::{OIDCClient, OIDCClientInfo};
use crate::auth::OIDCError;
use crate::cache::CachePool;
use crate::err::AppError;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::routing::router;
use crate::settings::Settings;
use crate::Result;
use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::eyre::eyre;
use log::info;
use oauth2::reqwest::async_http_client;
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tonic::transport::Channel;

pub struct Application {
    port: u16,
    server: Serve<IntoMakeService<Router>, Router>,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self> {
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

    pub async fn run_until_stopped(self) -> Result<()> {
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
    pub oidc_clients: Vec<OIDCClient>,
    pub settings: Settings,
}

impl AppState {
    pub async fn new(
        db: PgPool,
        cache: CachePool,
        agency_service: AgencyServiceClient<Channel>,
        oidc_clients: Vec<OIDCClient>,
        settings: Settings,
    ) -> Result<Self> {
        Ok(Self {
            db,
            cache,
            agency_service,
            oidc_clients,
            settings,
        })
    }
    pub async fn initialize(settings: Settings) -> Result<Self> {
        Ok(Self {
            db: db_connect(settings.db.expose()).await?,
            cache: CachePool::new(&settings.cache).await?,
            agency_service: agency_service_connect(settings.agency_api.expose()).await?,
            oidc_clients: initialize_oidc_clients(settings.oidc.clone()).await?,
            settings,
        })
    }
}

async fn initialize_oidc_clients(oidc: Vec<OIDCClientInfo>) -> Result<Vec<OIDCClient>> {
    let mut clients: Vec<OIDCClient> = vec![];
    for oidc_info in oidc {
        let issuer = oidc_info.issuer.clone();

        let metadata =
            CoreProviderMetadata::discover_async(oidc_info.issuer_url.clone(), async_http_client)
                .await
                .map_err(OIDCError::Discovery)?;
        let client = CoreClient::from_provider_metadata(
            metadata,
            oidc_info.client_id,
            Some(oidc_info.client_secret),
        );
        clients.push(OIDCClient { issuer, client })
    }
    Ok(clients)
}

pub async fn db_connect(database_url: &str) -> Result<PgPool> {
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(eyre!("Failed to connect to Postgres: {}", e).into()),
    }
}

pub async fn agency_service_connect(
    agency_service_url: &str,
) -> Result<AgencyServiceClient<Channel>> {
    let agency_service = AgencyServiceClient::connect(agency_service_url.to_owned())
        .await
        .map_err(|e| eyre!("Failed to connect to agency service: {}", e))?;

    Ok(agency_service)
}

async fn run(
    listener: TcpListener,
    settings: Settings,
) -> Result<Serve<IntoMakeService<Router>, Router>> {
    let state = AppState::initialize(settings).await?;
    sqlx::migrate!()
        .run(&state.db)
        .await
        .map_err(AppError::from)?;

    let app = router(state)?;

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}

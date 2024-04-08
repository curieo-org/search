use crate::auth::oauth2::OAuth2Client;
use crate::cache::{Cache, CacheSettings};
use crate::routing::router;
use crate::settings::Settings;
use crate::Result;
use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::eyre::eyre;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

pub struct Application {
    port: u16,
    server: Serve<IntoMakeService<Router>, Router>,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self> {
        let address = format!("{}:{}", settings.host, settings.port);

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
    pub cache: Cache,
    pub oauth2_clients: Vec<OAuth2Client>,
    pub settings: Settings,
}

impl From<(PgPool, Cache, Settings)> for AppState {
    fn from((db, cache, settings): (PgPool, Cache, Settings)) -> Self {
        Self {
            db,
            cache,
            oauth2_clients: settings.oauth2_clients.clone(),
            settings,
        }
    }
}

pub async fn db_connect(database_url: &str) -> Result<PgPool> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(eyre!("Failed to connect to Postgres: {}", e).into()),
    }
}

pub async fn cache_connect(cache_settings: &CacheSettings) -> Result<Cache> {
    let cache_client = Cache::new(cache_settings).await?;

    Ok(cache_client)
}

async fn run(
    listener: TcpListener,
    settings: Settings,
) -> Result<Serve<IntoMakeService<Router>, Router>> {
    let db = db_connect(settings.db.expose()).await?;

    let cache = cache_connect(&settings.cache).await?;

    let state = AppState::from((db, cache, settings));

    let app = router(state)?;

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}

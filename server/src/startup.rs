use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::eyre::eyre;
use redis::Client as RedisClient;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routing::router;
use crate::settings::Settings;
use crate::Result;

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
    pub cache: redis::Client,
    pub settings: Settings,
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

pub async fn cache_connect(cache_url: &str) -> Result<RedisClient> {
    match RedisClient::open(cache_url) {
        Ok(client) => Ok(client),
        Err(e) => Err(eyre!("Failed to connect to Redis: {}", e).into()),
    }
}

async fn run(
    listener: TcpListener,
    settings: Settings,
) -> Result<Serve<IntoMakeService<Router>, Router>> {
    let db = db_connect(settings.db.expose()).await?;

    let cache = cache_connect(settings.cache.expose()).await?;

    let state = AppState {
        db,
        cache,
        settings,
    };

    let app = router(state)?;

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}

use crate::routing::router;
use crate::settings::Settings;
use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::eyre::eyre;
use color_eyre::Result;
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

        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub settings: Settings,
}

async fn db_connect(database_url: &str) -> Result<PgPool> {
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(eyre!("Failed to connect to Postgres: {}", e)),
    }
}

async fn run(listener: TcpListener) -> Result<Serve<IntoMakeService<Router>, Router>> {
    let settings = Settings::new();
    let state = AppState {
        db: db_connect(&settings.db).await?,
        settings,
    };

    let app = router(state);

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}

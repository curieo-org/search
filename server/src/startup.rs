use crate::routing::router;
use crate::settings::Settings;
use axum::{extract::FromRef, routing::IntoMakeService, serve::Serve, Router};
use color_eyre::Result;
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
        let server = run(listener).await;

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
    pub settings: Settings,
}

async fn run(listener: TcpListener) -> Serve<IntoMakeService<Router>, Router> {
    let state = AppState {
        settings: Settings::new(),
    };

    let app = router(state);

    let server = axum::serve(listener, app.into_make_service());

    server
}

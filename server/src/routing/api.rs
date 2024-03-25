use axum::{routing::get, Router};
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::cookie::SameSite;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::health_check;
use crate::startup::AppState;
use crate::users::PostgresBackend;

pub fn router(state: AppState) -> color_eyre::Result<Router> {
    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    //sqlx::migrate!().run(&db).await?;

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string())?,
        Some(TokenUrl::new("http://token".to_string())?),
    );
    let backend = PostgresBackend::new(state.db.clone(), client);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
    let api_routes = Router::new()
        //.nest("/auth", auth::routes())
        //.nest("/search", search::routes())
        // Health check should be accessible regardless of session middleware
        //.layer(middleware::from_fn(some_auth_middleware))
        .merge(health_check::routes());

    Ok(Router::new()
        .route("/", get(|| async { "Hello, server!" }))
        .nest("/api", api_routes)
        .route_layer(login_required!(PostgresBackend, login_url = "/login"))
        .layer(auth_layer)
        .merge(health_check::routes())
        .with_state(state.clone())
        // TODO: CORS should be configurable via settings
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        ))
}

use axum::Router;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::cookie::SameSite;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::auth::models::{OAuth2Clients, PostgresBackend};
use crate::startup::AppState;
use crate::{auth, health_check, search, users};

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
    let oauth_clients = OAuth2Clients::default();
    let backend = PostgresBackend::new(state.db.clone(), oauth_clients);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
    let api_routes = Router::new()
        //.nest("/search", search::routes())
        //.layer(middleware::from_fn(some_auth_middleware))
        .nest("/search", search::routes())
        .nest("/users", users::routes())
        .route_layer(login_required!(
            PostgresBackend,
            login_url = "/api/auth/login"
        ))
        // Health check should be accessible regardless of session middleware
        .merge(health_check::routes())
        .nest("/auth", auth::routes());

    Ok(Router::new()
        .nest("/api", api_routes)
        .merge(health_check::routes())
        .with_state(state.clone())
        .layer(auth_layer)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        ))
}

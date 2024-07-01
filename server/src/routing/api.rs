use axum::Router;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::cookie::SameSite;
use axum_login::tower_sessions::{CachingSessionStore, Expiry, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::auth::models::PostgresBackend;
use crate::auth::sessions::{DashStore, RedisStore};
use crate::startup::AppState;
use crate::{auth, health_check, search, users};

pub fn router(state: AppState) -> color_eyre::Result<Router> {
    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = RedisStore::new(state.cache.clone());
    let caching_session_store = CachingSessionStore::new(DashStore::default(), session_store);
    let session_layer = SessionManagerLayer::new(caching_session_store)
        .with_secure(true)
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = PostgresBackend::new(state.db.clone(), state.oauth2_clients.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let api_routes = Router::new()
        .nest("/users", users::routes())
        .nest("/search", search::routes())
        .route_layer(login_required!(PostgresBackend, login_url = "/auth/login"))
        .nest("/auth", auth::routes());

    Ok(Router::new()
        .merge(api_routes)
        // Health check should be accessible regardless of session middleware
        .merge(health_check::routes())
        .with_state(state.clone())
        .layer(auth_layer)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction()))
}

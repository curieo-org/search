use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::startup::AppState;

pub fn router(with_state: AppState) -> Router {
    let api_routes = Router::new();
    //.nest("/auth", auth::routes())
    //.nest("/search", search::routes())
    // Health check should be accessible regardless of session middleware
    //.layer(middleware::from_fn(some_auth_middleware))
    //.merge(health_check::routes());

    Router::new()
        .route("/", get(|| async { "Hello, server!" }))
        .nest("/api", api_routes)
        .with_state(with_state.clone())
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

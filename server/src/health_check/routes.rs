use axum::routing::get;
use axum::Router;

use crate::health_check::selectors::health_check;
use crate::startup::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

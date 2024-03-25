use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "")
}

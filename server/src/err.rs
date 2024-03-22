use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::error;

pub enum AppError {
    GenericError(color_eyre::eyre::Error),
}

impl From<color_eyre::eyre::Error> for AppError {
    fn from(inner: color_eyre::eyre::Error) -> Self {
        AppError::GenericError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::GenericError(e) => {
                error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
            }
        };

        // TODO: We could(/should) use a specific struct here eventually.
        let body = Json(json!({ "error": error_message }));

        // Return a http status code and json body with error message.
        (status, body).into_response()
    }
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use secrecy::ExposeSecret;
use tower::ServiceExt;

use server::routing::router;
use server::settings::Settings;
use server::startup::{db_connect, AppState};

#[tokio::test]
async fn health_check_works() {
    let settings = Settings::new();

    let db = db_connect(settings.db.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let state = AppState { db, settings };

    let router = router(state).unwrap();
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

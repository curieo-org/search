use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use server::routing::router;
use server::settings::Settings;
use server::startup::{agency_service_connect, cache_connect, db_connect, AppState};

#[tokio::test]
async fn health_check_works() {
    let settings = Settings::new();

    let db = db_connect(settings.db.expose()).await.unwrap();
    let cache = cache_connect(&settings.cache).await.unwrap();
    let agency_service = agency_service_connect(&settings.agency_api).await.unwrap();
    let state = AppState::from((db, cache, settings, agency_service));

    let router = router(state).unwrap();
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use server::cache::CachePool;
use sqlx::PgPool;
use tower::ServiceExt;

use server::routing::router;
use server::settings::Settings;
use server::startup::{agency_service_connect, AppState};

#[sqlx::test]
async fn health_check_works(pool: PgPool) {
    let settings = Settings::new();
    let cache = CachePool::new(&settings.cache).await.unwrap();
    let agency_service = agency_service_connect(settings.agency_api.expose())
        .await
        .unwrap();
    let brave_api_config = settings.brave.clone().into();
    let state = AppState::new(
        pool,
        cache,
        agency_service,
        vec![],
        settings,
        brave_api_config,
        regex::Regex::new("").unwrap(),
    )
    .await
    .unwrap();
    let router = router(state).unwrap();
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

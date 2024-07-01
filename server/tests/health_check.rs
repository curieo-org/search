use axum::body::Body;
use axum::http::{Request, StatusCode};
use server::cache::CachePool;
use server::routing::router;
use server::settings::Settings;
use server::startup::AppState;
use sqlx::PgPool;
use tower::ServiceExt;
mod utils;

#[sqlx::test]
async fn health_check_works(pool: PgPool) {
    let settings = Settings::new();
    let cache = CachePool::new(&settings.cache).await.unwrap();
    let (_, agency_service) = utils::agency_server_and_client_stub().await;
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

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use server::cache::CacheSettings;
use server::routing::router;
use server::settings::Settings;
use server::startup::{cache_connect, db_connect, AppState};

#[tokio::test]
async fn health_check_works() {
    let settings = Settings::new();

    let db = db_connect(settings.db.expose()).await.unwrap();
    let cache_settings = CacheSettings {
        cache_url: settings.cache_url.expose().to_string(),
        enabled: settings.cache_enabled,
        ttl: settings.cache_ttl,
        max_sorted_size: settings.cache_max_sorted_size,
    };
    let cache = cache_connect(&cache_settings).await.unwrap();
    let state = AppState::from((db, cache, settings));

    let router = router(state).unwrap();
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

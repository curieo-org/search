use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, StatusCode};
use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::routing::router;
use server::settings::Settings;
use server::startup::AppState;
use server::startup::{agency_service_connect, cache_connect};
use server::users::selectors::get_user;
use server::Result;
use sqlx::PgPool;
use tower::ServiceExt;

/// Helper function to create a GET request for a given URI.
fn _send_get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .method("GET")
        .body(Body::empty())
        .unwrap()
}

#[sqlx::test]
async fn register_and_get_users_test(pool: PgPool) -> Result<()> {
    let user = get_user(pool.clone(), uuid::Uuid::nil()).await.unwrap();
    assert!(user.is_none());

    let new_user = register(
        pool.clone(),
        RegisterUserRequest {
            email: "test-email".to_string(),
            username: "test-username".to_string(),
            password: Some("password".to_string().into()),
            access_token: Default::default(),
        },
    )
    .await?;

    let user = get_user(pool.clone(), new_user.user_id).await?.unwrap();

    assert_eq!(user.user_id, new_user.user_id);
    assert_eq!(user.email, new_user.email);

    Ok(())
}

#[sqlx::test]
async fn register_users_works(pool: PgPool) {
    let settings = Settings::new();
    let cache = cache_connect(&settings.cache).await.unwrap();
    let agency_service = agency_service_connect(&settings.agency_api).await.unwrap();
    let state = AppState::from((pool, cache, settings, agency_service));
    let router = router(state).unwrap();

    let form = &[
        ("email", "my-email@email.com"),
        ("username", "my-username"),
        ("password", "my-password"),
    ];
    let serialized_body = serde_urlencoded::to_string(form).unwrap();
    let request = Request::post("/api/auth/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.clone().oneshot(request.clone()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Doing the same thing again should return a 422 status code.
    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let form = &[
        ("email", "another-email@email.com"),
        ("username", "another-username"),
        ("access_token", "my-access-token"),
    ];
    let serialized_body = serde_urlencoded::to_string(form).unwrap();
    let request = Request::post("/api/auth/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

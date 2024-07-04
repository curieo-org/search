use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, StatusCode};
use server::auth::{register, RegisterUserRequest, WhitelistedEmail};
use server::cache::CachePool;
use server::routing::router;
use server::settings::Settings;
use server::startup::AppState;
use server::users::selectors::get_user;
use server::users::services::whitelist_email;
use server::Result;
use sqlx::PgPool;
use tower::ServiceExt;

mod utils;

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
    let WhitelistedEmail { email, .. } =
        whitelist_email(&pool, "my-email@email.com").await.unwrap();

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

    let form = &[
        ("email", email.as_str()),
        ("username", "my-username"),
        ("password", "my-password"),
    ];
    let serialized_body = serde_urlencoded::to_string(form).unwrap();
    let request = Request::post("/auth/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.clone().oneshot(request.clone()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Doing the same thing again should return a 409 status code.
    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let form = &[
        ("email", "not-whitelisted-email"),
        ("username", "another-username"),
        ("access_token", "my-access-token"),
    ];
    let serialized_body = serde_urlencoded::to_string(form).unwrap();
    let request = Request::post("/auth/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

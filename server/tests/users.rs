use axum::body::Body;
use axum::extract::rejection::{
    FailedToDeserializeForm, FailedToDeserializeFormBody, FormRejection, RawFormRejection,
};
use axum::extract::{FromRequest, RawForm};
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::RequestExt;
use axum::{http, Form};
use sqlx::PgPool;
use tower::ServiceExt;

use server::auth::models::RegisterUserRequest;
use server::routing::router;
use server::secrets::Secret;
use server::settings::Settings;
use server::startup::{db_connect, AppState};
use server::users::selectors::get_user;
use server::users::services::register_user;
/// Helper function to create a GET request for a given URI.
fn send_get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .method("GET")
        .body(Body::empty())
        .unwrap()
}

#[sqlx::test]
async fn register_and_get_users_test(pool: PgPool) -> color_eyre::Result<()> {
    let user = get_user(pool.clone(), uuid::Uuid::nil()).await.unwrap();
    assert!(user.is_none());

    let new_user = register_user(
        pool.clone(),
        RegisterUserRequest {
            email: "test-email".to_string(),
            username: "test-username".to_string(),
            password_hash: Some("password".to_string()).into(),
            access_token: Default::default(),
        },
    )
    .await?
    .unwrap();

    let user = get_user(pool.clone(), new_user.user_id).await?.unwrap();

    assert_eq!(user.user_id, new_user.user_id);
    assert_eq!(user.email, new_user.email);

    Ok(())
}

#[tokio::test]
async fn register_users_works() {
    let settings = Settings::new();

    let db = db_connect(settings.db.expose())
        .await
        .expect("Failed to connect to Postgres.");

    let state = AppState { db, settings };
    let router = router(state).unwrap();

    let form = &[
        ("email", "my-email@email.com"),
        ("username", "my-username"),
        ("password_hash", "my-password"),
    ];
    let serialized_body = serde_urlencoded::to_string(&form).unwrap();
    let request = Request::post("/api/users/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let form = &[
        ("email", "my-email@email.com"),
        ("username", "my-username"),
        ("access_token", "my-access-token"),
    ];
    let serialized_body = serde_urlencoded::to_string(&form).unwrap();
    let request = Request::post("/api/users/register")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(serialized_body)
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

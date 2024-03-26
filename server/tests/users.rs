use sqlx::PgPool;

use server::users::selectors::get_user;
use server::users::services::create_user;
use server::users::CreateUserRequest;

#[sqlx::test]
async fn get_and_create_users_test(pool: PgPool) -> sqlx::Result<()> {
    let user = get_user(pool.clone(), uuid::Uuid::nil()).await.unwrap();
    assert!(user.is_none());

    let new_user = create_user(
        pool.clone(),
        CreateUserRequest {
            email: "test-email".to_string(),
            username: "test-username".to_string(),
            password_hash: Some("password".to_string()).into(),
            access_token: Default::default(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    let user = get_user(pool.clone(), new_user.user_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user_id, new_user.user_id);
    assert_eq!(user.email, new_user.email);

    Ok(())
}

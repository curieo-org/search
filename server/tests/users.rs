use sqlx::PgPool;

use server::users::selectors::get_user;
use server::users::services::create_user;
use server::users::CreateUser;

#[sqlx::test]
async fn get_users_test(pool: PgPool) -> sqlx::Result<()> {
    let user = get_user(pool.clone(), uuid::Uuid::nil()).await.unwrap();
    assert!(user.is_none());

    let new_user = create_user(
        pool.clone(),
        CreateUser {
            email: Default::default(),
            username: Default::default(),
            password_hash: Some("password".to_string()).into(),
            access_token: Default::default(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    let user = get_user(pool.clone(), new_user.user_id).await.unwrap();
    assert!(user.is_some());

    Ok(())
}

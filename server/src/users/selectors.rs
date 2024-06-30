use crate::users::{User, UserRecord};
use sqlx::types::uuid;
use sqlx::PgPool;

#[tracing::instrument(level = "info", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> crate::Result<Option<UserRecord>> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(&pool)
        .await?;

    Ok(user.map(UserRecord::from))
}

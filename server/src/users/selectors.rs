use sqlx::types::uuid;
use sqlx::PgPool;

use crate::users::{User, UserRecord};

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> crate::Result<Option<UserRecord>> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(&pool)
        .await?;

    Ok(user.map(UserRecord::from))
}

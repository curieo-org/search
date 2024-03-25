use sqlx::{Acquire, PgPool};
use sqlx::types::uuid;

use crate::users::User;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> color_eyre::Result<Option<User>> {
    let mut conn = pool.acquire().await?;
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id)
        .fetch_optional(conn.acquire().await?)
        .await?;

    Ok(user)
}

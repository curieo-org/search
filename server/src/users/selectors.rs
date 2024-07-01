use sqlx::types::uuid;
use sqlx::PgPool;

use crate::users::{User, UserRecord};

#[tracing::instrument(level = "info", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> color_eyre::Result<Option<UserRecord>> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(&pool)
        .await?;

    Ok(user.map(UserRecord::from))
}

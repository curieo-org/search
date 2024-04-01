use sqlx::types::uuid;
use sqlx::PgPool;

use crate::users::User;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> color_eyre::Result<Option<User>> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(&pool)
        .await?;

    Ok(user)
}

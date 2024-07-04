use crate::users::{User, UserError, UserRecord};
use sqlx::types::uuid;
use sqlx::PgPool;

#[tracing::instrument(level = "info", ret, err)]
pub async fn get_user(pool: PgPool, user_id: uuid::Uuid) -> Result<Option<UserRecord>, UserError> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(&pool)
        .await?;

    Ok(user.map(UserRecord::from))
}

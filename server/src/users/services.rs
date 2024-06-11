use crate::secrets::Secret;
use crate::auth::utils::hash_password;
use sqlx::PgPool;
use uuid::Uuid;


#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_password(
    pool: &PgPool,
    user_id: &Uuid,
    password: Secret<String>,
) -> crate::Result<()> {
    let password_hash = hash_password(password).await?;
    sqlx::query_as!(
      User,
      "update users set password_hash = $1 where user_id = $2",
      password_hash.expose(),
      user_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(());
}
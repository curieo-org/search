use color_eyre::eyre::eyre;
use sqlx::{Acquire, PgPool};

use crate::users::{CreateUser, User};

#[tracing::instrument(level = "debug", ret, err)]
async fn create_user(pool: PgPool, create_user: CreateUser) -> color_eyre::Result<Option<User>> {
    let mut conn = pool.acquire().await?;

    if let Some(password_hash) = create_user.password_hash.as_ref() {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING *",
            create_user.email,
            create_user.username,
            password_hash
        )
        .fetch_optional(conn.acquire().await?)
        .await?;

        return Ok(user);
    } else if let Some(access_token) = create_user.access_token.as_ref() {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, username, access_token) VALUES ($1, $2, $3) RETURNING *",
            create_user.email,
            create_user.username,
            access_token
        )
        .fetch_optional(conn.acquire().await?)
        .await?;

        return Ok(user);
    }

    Err(eyre!(
        "Either password_hash or access_token must be provided to create a user"
    ))
}

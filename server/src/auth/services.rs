use crate::auth::models::RegisterUserRequest;
use crate::auth::utils;
use crate::err::{AppError, ResultExt};
use color_eyre::eyre::eyre;
use sqlx::PgPool;

use crate::users::User;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn register(pool: PgPool, request: RegisterUserRequest) -> crate::Result<User> {
    if let Some(password) = request.password_hash {
        let password_hash = utils::hash_password(password).await;
        let user = sqlx::query_as!(
            User,
            "insert into users (email, username, password_hash) values ($1, $2, $3) returning *",
            request.email,
            request.username,
            password_hash.expose()
        )
        .fetch_one(&pool)
        .await
        .on_constraint("user_username_key", |_| {
            AppError::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            AppError::unprocessable_entity([("email", "email taken")])
        })?;

        return Ok(user);
    } else if let Some(access_token) = request.access_token {
        let user = sqlx::query_as!(
            User,
            "insert into users (email, username, access_token) values ($1, $2, $3) returning *",
            request.email,
            request.username,
            access_token.expose()
        )
        .fetch_one(&pool)
        .await
        .on_constraint("user_username_key", |_| {
            AppError::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            AppError::unprocessable_entity([("email", "email taken")])
        })?;

        return Ok(user);
    }
    Err(eyre!("Either password_hash or access_token must be provided to create a user").into())
}

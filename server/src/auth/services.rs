use crate::auth::models;
use crate::auth::utils;
use crate::err::{AppError, ResultExt};
use crate::users::{User, UserRecord};
use color_eyre::eyre::eyre;
use sqlx::PgPool;

#[tracing::instrument(level = "info", ret, err)]
pub async fn register(pool: PgPool, request: models::RegisterUserRequest) -> crate::Result<UserRecord> {
    if let Some(password) = request.password {
        let password_hash = utils::hash_password(password).await?;
        let user = sqlx::query_as!(
            User,
            "insert into users (email, username, password_hash) values ($1, $2, $3) returning *",
            request.email,
            request.username,
            password_hash.expose()
        )
        .fetch_one(&pool)
        .await
        .on_constraint("users_username_key", |_| {
            AppError::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("users_email_key", |_| {
            AppError::unprocessable_entity([("email", "email taken")])
        })?;

        return Ok(user.into());
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
        .on_constraint("users_username_key", |_| {
            AppError::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("users_email_key", |_| {
            AppError::unprocessable_entity([("email", "email taken")])
        })?;

        return Ok(user.into());
    }
    Err(eyre!("Either password or access_token must be provided to create a user").into())
}

pub async fn is_email_whitelisted(pool: &PgPool, email: &String) -> crate::Result<bool> {
    let whitelisted_email = sqlx::query_as!(models::WhitelistedEmail, "SELECT * FROM whitelisted_emails WHERE email = $1", email)
    .fetch_one(pool)
    .await;

    match whitelisted_email {
        Ok(whitelisted_email) => Ok(whitelisted_email.approved),
        _ => Ok(false)
    }
 }

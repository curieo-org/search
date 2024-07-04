use crate::auth::AuthError;
use crate::auth::{api_models, models, utils};
use crate::err::ResultExt;
use crate::users::{User, UserRecord};
use sqlx::PgPool;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn register(
    pool: PgPool,
    request: api_models::RegisterUserRequest,
) -> crate::Result<UserRecord> {
    if let Some(password) = request.password {
        let password_hash = utils::hash_password(password).await?;
        let user = sqlx::query_as!(
            User,
            "insert into users (email, username, password_hash) values ($1, $2, $3) returning *",
            request.email,
            request.email,
            password_hash.expose()
        )
        .fetch_one(&pool)
        .await
        .on_constraint("users_username_key", |_| {
            AuthError::UserAlreadyExists(format!(
                "User with username {} already exists",
                request.email
            ))
            .into()
        })
        .on_constraint("users_email_key", |_| {
            AuthError::UserAlreadyExists(format!(
                "User with email {} already exists",
                request.email
            ))
            .into()
        })?;

        return Ok(user.into());
    } else if let Some(access_token) = request.access_token {
        let user = sqlx::query_as!(
            User,
            "insert into users (email, username, access_token) values ($1, $2, $3) returning *",
            request.email,
            request.email,
            access_token.expose()
        )
        .fetch_one(&pool)
        .await
        .on_constraint("users_username_key", |_| {
            AuthError::UserAlreadyExists(format!(
                "User with username {} already exists",
                request.email
            ))
            .into()
        })
        .on_constraint("users_email_key", |_| {
            AuthError::UserAlreadyExists(format!(
                "User with email {} already exists",
                request.email
            ))
            .into()
        })?;

        return Ok(user.into());
    }

    Err(AuthError::InvalidData(
        "Either password or access_token must be provided to create a user".to_string(),
    )
    .into())
}

pub async fn is_email_whitelisted(pool: &PgPool, email: &String) -> Result<bool, AuthError> {
    let whitelisted_email = sqlx::query_as!(
        models::WhitelistedEmail,
        "SELECT * FROM whitelisted_emails WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await;

    match whitelisted_email {
        Err(_) => Ok(false),
        Ok(whitelisted_email) => Ok(whitelisted_email.approved),
    }
}

use crate::auth::utils::hash_password;
use crate::auth::WhitelistedEmail;
use crate::secrets::Secret;
use crate::users::{api_models, models, UserError};
use sqlx::PgPool;
use uuid::Uuid;

type Result<T> = std::result::Result<T, UserError>;

#[tracing::instrument(level = "info", ret, err)]
pub async fn update_profile(
    pool: &PgPool,
    user_id: &Uuid,
    update_profile_request: api_models::UpdateProfileRequest,
) -> Result<models::User> {
    let user = sqlx::query_as!(
        models::User,
        "
        update users
        set

          fullname = coalesce($1::text, fullname),
          title = coalesce($2::text, title),
          company = coalesce($3::text, company)
          where user_id = $4 returning *
        ",
        update_profile_request.fullname,
        update_profile_request.title,
        update_profile_request.company,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(user);
}

#[tracing::instrument(level = "info", ret, err)]
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
    .execute(pool)
    .await?;

    return Ok(());
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn whitelist_email(pool: &PgPool, email: &str) -> crate::Result<WhitelistedEmail> {
    Ok(sqlx::query_as!(
        WhitelistedEmail,
        "insert into whitelisted_emails (email, approved) values ($1, true) returning *",
        email,
    )
    .fetch_one(pool)
    .await?)
}

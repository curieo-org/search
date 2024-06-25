use crate::auth::utils::hash_password;
use crate::secrets::Secret;
use crate::users::models;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_profile(
    pool: &PgPool,
    user_id: &Uuid,
    update_profile_request: models::UpdateProfileRequest,
) -> crate::Result<models::User> {
    let user = sqlx::query_as!(
        models::User,
        "
      update users
      set
        username = coalesce($1::text, username),
        email = coalesce($2::text, email),
        fullname = coalesce($3::text, fullname),
        title = coalesce($4::text, title),
        company = coalesce($5::text, company)
      where user_id = $6 returning *
      ",
        update_profile_request.username,
        update_profile_request.email,
        update_profile_request.fullname,
        update_profile_request.title,
        update_profile_request.company,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(user);
}

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
    .execute(pool)
    .await?;

    return Ok(());
}

use crate::secrets::Secret;
use crate::auth::utils::hash_password;
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
      set username = case 
              when $1::text is not null then $1::text
              else username
          end,
          email = case 
              when $2::text is not null then $2::text
              else email
          end,
          fullname = case 
              when $3::text is not null then $3::text
              else fullname
          end,
          title = case 
              when $4::text is not null then $4::text
              else title
          end,
          company = case 
              when $5::text is not null then $5::text
              else company
          end
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
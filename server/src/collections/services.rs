use crate::collections::{api_models, data_models};
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_new_collection(
    pool: &PgPool,
    user_id: &Uuid,
    create_collections_request: &api_models::CreateCollectionRequest,
) -> crate::Result<data_models::Collection> {
    let collection = sqlx::query_as!(
        data_models::Collection,
        "insert into collections (user_id, name, description, category) values ($1, $2, $3, $4) returning *",
        user_id,
        create_collections_request.name,
        create_collections_request.description,
        i32::from(create_collections_request.category),
    )
    .fetch_one(pool)
    .await?;

    return Ok(collection);
}

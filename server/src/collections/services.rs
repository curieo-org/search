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
        create_collections_request.category as i32,
    )
    .fetch_one(pool)
    .await?;

    return Ok(collection);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_collection(
    pool: &PgPool,
    user_id: &Uuid,
    update_collection_request: &api_models::UpdateCollectionRequest,
) -> crate::Result<data_models::Collection> {
    let collection = sqlx::query_as!(
        data_models::Collection,
        "
        update collections
        set name = case 
                when $1::text is not null then $1::text
                else name
            end,
            description = case 
                when $2::text is not null then $2::text
                else description
            end,
            category = case 
                when $3::integer is not null then $3::integer
                else category
            end
        where collection_id = $4 returning *
        ",
        update_collection_request.name,
        update_collection_request.description,
        update_collection_request.category.map(|category| category as i32),
        update_collection_request.collection_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(collection);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_collections(
    pool: &PgPool,
    user_id: &Uuid,
    get_collections_request: &api_models::GetCollectionsRequest,
) -> crate::Result<api_models::GetCollectionsResponse> {
    let collections = sqlx::query_as!(
        data_models::Collection,
        "select * from collections where user_id = $1 order by created_at desc limit $2 offset $3",
        user_id,
        get_collections_request.limit.unwrap_or(10) as i64,
        get_collections_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    return Ok(api_models::GetCollectionsResponse { collections });
}

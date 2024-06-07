use crate::collections::{api_models, data_models};
use crate::search::data_models::{Search, Source};
use sqlx::{PgPool, Error};
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
                when $3::int is not null then $3::int
                else category
            end
        where collection_id = $4 returning *
        ",
        update_collection_request.name,
        update_collection_request.description,
        update_collection_request.category.map(|s| s as i32),
        update_collection_request.collection_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(collection);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn delete_collection(
    pool: &PgPool,
    user_id: &Uuid,
    delete_collection_request: &api_models::DeleteCollectionRequest,
) -> Result<(), Error> {
    sqlx::query_as!(
        data_models::Collection,
        "delete from collections where collection_id = $1",
        delete_collection_request.collection_id,
    )
    .execute(pool)
    .await?;

    return Ok(());
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

#[tracing::instrument(level = "debug", ret, err)]
pub async fn add_items_to_collection(
    pool: &PgPool,
    user_id: &Uuid,
    add_items_to_collection_request: &api_models::AddItemsToCollectionRequest,
) -> Result<(), Error> {
    // check if items are actually search/source
    sqlx::query_as!(
        data_models::CollectionItems,
        "insert into collection_items (collection_id, item_id, item_type) select $1, unnest($2::uuid[]), unnest($3::int[])",
        add_items_to_collection_request.collection_id,
        &add_items_to_collection_request.items.iter().map(|item| item.item_id).collect::<Vec<uuid::Uuid>>(),
        &add_items_to_collection_request.items.iter().map(|item| item.item_type as i32).collect::<Vec<i32>>(),
    )
    .execute(pool)
    .await?;

    return Ok(());
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_items_from_collection(
    pool: &PgPool,
    user_id: &Uuid,
    get_items_from_collection_request: &api_models::GetItemsFromCollectionRequest,
) -> crate::Result<api_models::GetItemsFromCollectionResponse> {
    let items = sqlx::query_as!(
        data_models::CollectionItem,
        "select * from collection_items where collection_id = $1 and item_type = $2 order by created_at desc limit $3 offset $4",
        get_items_from_collection_request.collection_id,
        get_items_from_collection_request.item_type as i32,
        get_items_from_collection_request.limit.unwrap_or(10) as i64,
        get_items_from_collection_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    // if get_items_from_collection_request.item_type as i32 == 0 {
    //     let searches = sqlx::query_as!(
    //         Search,
    //         "select s.* from searches s join collection_items i on s.search_id = i.item_id where i.collection_id = $1",
    //         get_items_from_collection_request.collection_id,
    //     )
    //     .fetch_all(pool)
    //     .await?;
    // }

    // if get_items_from_collection_request.item_type as i32 == 1 {
    //     let sources = sqlx::query_as!(
    //         Source,
    //         "select s.* from sources s join collection_items i on s.source_id = i.item_id where i.collection_id = $1",
    //         get_items_from_collection_request.collection_id,
    //     )
    //     .fetch_all(pool)
    //     .await?;
    // }

    return Ok(api_models::GetItemsFromCollectionResponse { items });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn delete_items_from_collection(
    pool: &PgPool,
    user_id: &Uuid,
    delete_items_from_collection_request: &api_models::DeleteItemsFromCollectionRequest,
) -> Result<(), Error>  {
    sqlx::query_as!(
        data_models::CollectionItem,
        "delete from collection_items where collection_id = $1 and item_id in (select unnest($2::uuid[]))",
        delete_items_from_collection_request.collection_id,
        &delete_items_from_collection_request.items
    )
    .execute(pool)
    .await?;

    return Ok(());
}

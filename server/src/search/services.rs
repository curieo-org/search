use crate::proto::SearchResponse as AgencySearchResponse;
use crate::search::{api_models, data_models};
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_new_search(
    pool: &PgPool,
    user_id: &Uuid,
    search_query: &api_models::SearchQueryRequest,
) -> crate::Result<data_models::Search> {
    let thread = match search_query.thread_id {
        Some(thread_id) => {
            sqlx::query_as!(
                data_models::Thread,
                "select * from threads where thread_id = $1",
                thread_id,
            )
            .fetch_one(pool)
            .await?
        }
        None => {
            sqlx::query_as!(
                data_models::Thread,
                "insert into threads (user_id, title) values ($1, $2) returning *",
                &user_id,
                &search_query.query,
            )
            .fetch_one(pool)
            .await?
        }
    };

    let search = sqlx::query_as!(
        data_models::Search,
        "insert into searches (thread_id, query, result) values ($1, $2, $3) returning *",
        &thread.thread_id,
        search_query.query,
        &String::from(""),
    )
    .fetch_one(pool)
    .await?;

    return Ok(search);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_search_result(
    pool: &PgPool,
    user_id: &Uuid,
    search: &data_models::Search,
    search_response: &AgencySearchResponse,
) -> crate::Result<api_models::SearchByIdResponse> {
    let sources = sqlx::query_as!(
        data_models::Source,
        "insert into sources (title, description, url, source_type, metadata) \
            select * from unnest($1::text[], $2::text[], $3::text[], $4::int[], $5::jsonb[]) \
            on conflict (url) do update set title = excluded.title, description = excluded.description, \
            source_type = excluded.source_type returning *",
        &search_response.sources.iter().map(|s| s.title.clone()).collect::<Vec<String>>(),
        &search_response.sources.iter().map(|s| s.description.clone()).collect::<Vec<String>>(),
        &search_response.sources.iter().map(|s| s.url.clone()).collect::<Vec<String>>(),
        &search_response.sources.iter().map(|s| s.source_type as i32).collect::<Vec<i32>>(),
        &search_response.sources.iter().map(|s| serde_json::to_value(s.metadata.clone()).unwrap()).collect::<Vec<serde_json::Value>>(),
    )
    .fetch_all(pool)
    .await?;

    sqlx::query!(
        "insert into search_sources (search_id, source_id) \
            select * from unnest($1::uuid[], $2::uuid[])",
        &vec![search.search_id; sources.len()],
        &sources.iter().map(|s| s.source_id).collect::<Vec<Uuid>>(),
    )
    .fetch_all(pool)
    .await?;

    let updated_search = sqlx::query_as!(
        data_models::Search,
        "update searches set result = $1 where search_id = $2 returning *",
        search_response.result,
        search.search_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(api_models::SearchByIdResponse {
        result: updated_search.result,
        sources,
    });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_one_search(
    pool: &PgPool,
    user_id: &Uuid,
    search_by_id_request: &api_models::SearchByIdRequest,
) -> crate::Result<api_models::SearchByIdResponse> {
    let search = sqlx::query_as!(
        data_models::Search,
        "select * from searches where search_id = $1 and thread_id in \
            (select thread_id from threads where user_id = $2)",
        search_by_id_request.search_id,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    let sources = sqlx::query_as!(
        data_models::Source,
        "select * from sources where source_id in \
            (select source_id from search_sources where search_id = $1)",
        search.search_id,
    )
    .fetch_all(pool)
    .await?;

    return Ok(api_models::SearchByIdResponse {
        result: search.result,
        sources,
    });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_threads(
    pool: &PgPool,
    user_id: &Uuid,
    search_history_request: &api_models::SearchHistoryRequest,
) -> crate::Result<api_models::SearchHistoryResponse> {
    let threads = sqlx::query_as!(
        data_models::Thread,
        "select * from threads where user_id = $1 order by created_at desc limit $2 offset $3",
        user_id,
        search_history_request.limit.unwrap_or(10) as i64,
        search_history_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    return Ok(api_models::SearchHistoryResponse { threads });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_one_thread(
    pool: &PgPool,
    user_id: &Uuid,
    thread_by_id_request: &api_models::SearchThreadRequest,
) -> crate::Result<api_models::SearchThreadResponse> {
    let searches = sqlx::query_as!(
        data_models::Search,
        "select * from searches where thread_id in \
            (select thread_id from threads where thread_id = $1 and user_id = $2) \
            order by created_at desc limit $3 offset $4",
        thread_by_id_request.thread_id,
        user_id,
        thread_by_id_request.limit.unwrap_or(10) as i64,
        thread_by_id_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    let sources = sqlx::query_as!(
        data_models::Source,
        "select * from sources where source_id in \
            (select source_id from search_sources where search_id = any($1::uuid[]))",
        &searches.iter().map(|s| s.search_id).collect::<Vec<Uuid>>(),
    )
    .fetch_all(pool)
    .await?;

    let searches = searches
        .into_iter()
        .map(|s| {
            let sources = sources
                .iter()
                .filter(|source| {
                    sources
                        .iter()
                        .any(|search_source| search_source.source_id == source.source_id)
                })
                .cloned()
                .collect::<Vec<data_models::Source>>();
            api_models::SearchByIdResponse {
                result: s.result,
                sources,
            }
        })
        .collect::<Vec<api_models::SearchByIdResponse>>();

    return Ok(api_models::SearchThreadResponse { searches });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_thread(
    pool: &PgPool,
    user_id: &Uuid,
    thread_update_request: &api_models::UpdateThreadRequest,
) -> crate::Result<data_models::Thread> {
    let thread = sqlx::query_as!(
        data_models::Thread,
        "update threads set title = $1 where thread_id = $2 and user_id = $3 returning *",
        thread_update_request.title,
        thread_update_request.thread_id,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(thread);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_search_reaction(
    pool: &PgPool,
    user_id: &Uuid,
    search_reaction_request: &api_models::SearchReactionRequest,
) -> crate::Result<data_models::Search> {
    let search = sqlx::query_as!(
        data_models::Search,
        "update searches set reaction = $1 where search_id = $2 returning *",
        search_reaction_request.reaction,
        search_reaction_request.search_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(search);
}

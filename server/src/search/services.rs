use crate::err::AppError;
use crate::rag::Source;
use crate::search::{api_models, data_models};
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_new_search(
    pool: &PgPool,
    user_id: &Uuid,
    search_query_request: &api_models::SearchQueryRequest,
    rephrased_query: &String,
) -> crate::Result<data_models::Search> {
    let thread = match search_query_request.thread_id {
        Some(thread_id) => {
            sqlx::query_as!(
                data_models::Thread,
                "select * from threads where thread_id = $1 and user_id = $2",
                thread_id,
                user_id,
            )
            .fetch_one(pool)
            .await?
        }
        None => {
            sqlx::query_as!(
                data_models::Thread,
                "insert into threads (user_id, title) values ($1, $2) returning *",
                &user_id,
                &search_query_request.query,
            )
            .fetch_one(pool)
            .await?
        }
    };

    let search = sqlx::query_as!(
        data_models::Search,
        "insert into searches (thread_id, query, rephrased_query, result) values ($1, $2, $3, $4) returning *",
        &thread.thread_id,
        search_query_request.query,
        rephrased_query,
        &String::from(""),
    )
    .fetch_one(pool)
    .await?;

    return Ok(search);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn append_search_result(
    pool: &PgPool,
    search: &data_models::Search,
    result_suffix: &String,
) -> crate::Result<data_models::Search> {
    // Only used by internal services, so no need to check if user_id is the owner of the search
    let search = sqlx::query_as!(
        data_models::Search,
        "update searches set result = result || $1 where search_id = $2 returning *",
        result_suffix,
        search.search_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(search)
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn add_search_sources(
    pool: &PgPool,
    search: &data_models::Search,
    sources: &Vec<Source>,
) -> crate::Result<Vec<data_models::Source>> {
    if sources.len() == 0 {
        return Err(AppError::BadRequest("No sources to add".to_string()));
    }

    // remove duplicates with same url
    let mut hash_set: HashSet<&String> = sources.iter().map(|s| &s.url).collect();
    let sources = sources
        .into_iter()
        .filter(|s| match hash_set.contains(&s.url) {
            true => {
                hash_set.remove(&s.url);
                true
            }
            false => false,
        })
        .collect::<Vec<_>>();

    // Only used by internal services, so no need to check if user_id is the owner of the search
    let sources = sqlx::query_as!(
        data_models::Source,
        "insert into sources (title, description, url, source_type, metadata) \
            select * from unnest($1::text[], $2::text[], $3::text[], $4::int[], $5::jsonb[]) \
            on conflict (url) do update set title = excluded.title, description = excluded.description, \
            source_type = excluded.source_type, metadata = excluded.metadata returning *",
        &sources.iter().map(|s| s.title.clone()).collect::<Vec<String>>(),
        &sources.iter().map(|s| s.description.clone()).collect::<Vec<String>>(),
        &sources.iter().map(|s| s.url.clone()).collect::<Vec<String>>(),
        &sources.iter().map(|s| s.source_type.clone() as i32).collect::<Vec<i32>>(),
        &sources.iter().map(|s| serde_json::to_value(
            s.metadata.clone()
        ).unwrap_or(serde_json::json!({}))).collect::<Vec<serde_json::Value>>(),
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

    return Ok(sources);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_one_search(
    pool: &PgPool,
    user_id: &Uuid,
    search_by_id_request: &api_models::SearchByIdRequest,
) -> crate::Result<api_models::SearchByIdResponse> {
    let search = sqlx::query_as!(
        data_models::Search,
        "select s.* from searches s \
            inner join threads t on s.thread_id = t.thread_id \
            where s.search_id = $1 and t.user_id = $2",
        search_by_id_request.search_id,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    let sources = sqlx::query_as!(
        data_models::Source,
        "select s.* from sources s \
            inner join search_sources ss on s.source_id = ss.source_id \
            where ss.search_id = $1",
        search.search_id,
    )
    .fetch_all(pool)
    .await?;

    return Ok(api_models::SearchByIdResponse { search, sources });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_last_n_searches(
    pool: &PgPool,
    last_n: u8,
    thread_id: &Uuid,
) -> crate::Result<Vec<data_models::Search>> {
    // Only used by internal services, so no need to check if user_id is the owner of the search
    let searches = sqlx::query_as!(
        data_models::Search,
        "select s.* from searches s \
            where s.thread_id = $1 \
            order by s.created_at desc limit $2",
        thread_id,
        last_n as i64,
    )
    .fetch_all(pool)
    .await?;

    return Ok(searches);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_threads(
    pool: &PgPool,
    user_id: &Uuid,
    thread_history_request: &api_models::ThreadHistoryRequest,
) -> crate::Result<api_models::ThreadHistoryResponse> {
    let threads = sqlx::query_as!(
        data_models::Thread,
        "select * from threads where user_id = $1 order by created_at desc limit $2 offset $3",
        user_id,
        thread_history_request.limit.unwrap_or(10) as i64,
        thread_history_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    return Ok(api_models::ThreadHistoryResponse { threads });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn get_one_thread(
    pool: &PgPool,
    user_id: &Uuid,
    thread_by_id_request: &api_models::GetThreadRequest,
) -> crate::Result<api_models::SearchThreadResponse> {
    let thread = sqlx::query_as!(
        data_models::Thread,
        "select * from threads where thread_id = $1 and user_id = $2",
        thread_by_id_request.thread_id,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    let searches = sqlx::query_as!(
        data_models::Search,
        "select * from searches where thread_id = $1 \
            order by created_at desc limit $2 offset $3",
        thread.thread_id,
        thread_by_id_request.limit.unwrap_or(10) as i64,
        thread_by_id_request.offset.unwrap_or(0) as i64
    )
    .fetch_all(pool)
    .await?;

    let sources = sqlx::query_as!(
        data_models::Source,
        "select s.* from sources s \
            inner join search_sources ss on s.source_id = ss.source_id \
            where ss.search_id = any($1::uuid[])",
        &searches.iter().map(|s| s.search_id).collect::<Vec<Uuid>>(),
    )
    .fetch_all(pool)
    .await?;

    let searches = searches
        .into_iter()
        .map(|search| {
            let sources = sources
                .iter()
                .filter(|source| {
                    sources
                        .iter()
                        .any(|search_source| search_source.source_id == source.source_id)
                })
                .cloned()
                .collect::<Vec<data_models::Source>>();
            api_models::SearchByIdResponse { search, sources }
        })
        .collect::<Vec<api_models::SearchByIdResponse>>();

    return Ok(api_models::SearchThreadResponse { thread, searches });
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn update_thread(
    pool: &PgPool,
    user_id: &Uuid,
    update_thread_request: &api_models::UpdateThreadRequest,
) -> crate::Result<data_models::Thread> {
    let thread = sqlx::query_as!(
        data_models::Thread,
        "update threads set title = $1 where thread_id = $2 and user_id = $3 returning *",
        update_thread_request.title,
        update_thread_request.thread_id,
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
        "update searches s set reaction = $1 from threads t \
            where s.search_id = $2 and s.thread_id = t.thread_id and t.user_id = $3 \
            returning s.*",
        search_reaction_request.reaction,
        search_reaction_request.search_id,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    return Ok(search);
}

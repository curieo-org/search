use crate::err::AppError;
use crate::search::{RAGTokenResponse, SearchHistory, SearchQueryRequest, SearchResponse};
use crate::settings::SETTINGS;
use color_eyre::eyre::eyre;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use reqwest::Client as ReqwestClient;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(level = "debug", ret, err)]
pub async fn search(
    cache: &mut MultiplexedConnection,
    search_query: &SearchQueryRequest,
) -> crate::Result<SearchResponse> {
    let cache_response: Option<SearchResponse> = cache
        .get(search_query.query.clone())
        .await
        .map(|response: Option<String>| {
            response.and_then(|response| serde_json::from_str(&response).ok())
        })
        .map_err(|e| AppError::from(e))?;

    if let Some(response) = cache_response {
        return Ok(response);
    }

    // TODO: replace this with actual search logic using GRPC calls with backend services
    let rag_api_url = SETTINGS.rag_api.clone() + "/token";
    let form_data = [
        ("username", &SETTINGS.rag_api_username.expose()),
        ("password", &SETTINGS.rag_api_password.expose()),
    ];
    let token: RAGTokenResponse = ReqwestClient::new()
        .post(rag_api_url)
        .form(&form_data)
        .send()
        .await
        .map_err(|_| eyre!("unable to send request to rag api"))?
        .json()
        .await
        .map_err(|_| eyre!("unable to parse json response from rag api"))?;

    let rag_api_url = SETTINGS.rag_api.clone() + "/search?query=" + &search_query.query;
    let response: SearchResponse = ReqwestClient::new()
        .get(rag_api_url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .map_err(|_| eyre!("unable to send request to rag api"))?
        .json()
        .await
        .map_err(|_| eyre!("unable to parse json response from rag api"))?;

    return Ok(response);
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn insert_search_history(
    pool: &PgPool,
    cache: &mut MultiplexedConnection,
    user_id: &Uuid,
    search_query: &SearchQueryRequest,
    search_response: &SearchResponse,
) -> crate::Result<SearchHistory> {
    cache
        .set(
            &search_query.query,
            serde_json::to_string(&search_response)
                .map_err(|_| eyre!("unable to convert string to json"))?,
        )
        .await
        .map_err(|e| AppError::from(e))?;

    let search_history = sqlx::query_as!(
        SearchHistory,
        "insert into search_history (user_id, query, result, sources) values ($1, $2, $3, $4) returning *",
        user_id,
        search_query.query,
        search_response.result,
        &search_response.sources
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::from(e))?;

    return Ok(search_history);
}

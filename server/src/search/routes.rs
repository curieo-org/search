use crate::llms;
use crate::rag::{self, post_process, pre_process};
use crate::search::{api_models, data_models, services, SearchError};
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::{get, patch};
use axum::{Json, Router};
use futures::{stream::StreamExt, Stream};
use sqlx::PgPool;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use utoipa::OpenApi;
use validator::Validate;

#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn get_search_query_handler(
    State(AppState {
        cache,
        agency_service,
        settings,
        brave_config,
        openai_stream_regex,
        ..
    }): State<AppState>,
    State(pool): State<PgPool>,
    user: User,
    Query(search_query_request): Query<api_models::SearchQueryRequest>,
) -> crate::Result<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    search_query_request
        .validate()
        .map_err(|e| SearchError::InvalidData(format!("Invalid search query: {}", e)))?;
    let user_id = user.user_id;

    let (query_toxicity, rephrased_query) = tokio::join!(
        llms::toxicity::predict_toxicity(
            &settings.llm,
            llms::ToxicityInput {
                inputs: search_query_request.query.to_string(),
            }
        ),
        pre_process::rephrase_query(&pool, &settings, &search_query_request)
    );

    if let Ok(true) = query_toxicity {
        return Err(SearchError::ToxicQuery("Query is too toxic to proceed".to_string()).into());
    }
    let rephrased_query = match rephrased_query {
        Ok(rephrased_query) => rephrased_query,
        _ => search_query_request.query.clone(),
    };

    let (search_item, search_response) = tokio::join!(
        services::insert_new_search(&pool, &user_id, &search_query_request, &rephrased_query),
        rag::search(
            &settings,
            &brave_config,
            &cache,
            &agency_service,
            &rephrased_query
        )
    );
    let search_item = search_item?;
    let search_response = search_response?;

    let sources =
        services::add_search_sources(&pool, &search_item, &search_response.sources).await?;

    let (tx, rx) = mpsc::channel(1);
    tx.send(api_models::SearchByIdResponse {
        search: search_item.clone(),
        sources,
    })
    .await
    .map_err(|e| SearchError::Other(format!("Failed to send search result: {}", e)))?;

    let update_processor = api_models::UpdateResultProcessor::new(Arc::new(move |result_suffix| {
        let pool_clone = pool.clone();
        let search_item_clone = search_item.clone();
        Box::pin(async move {
            let search =
                services::append_search_result(&pool_clone, &search_item_clone, &result_suffix)
                    .await?;
            Ok(search)
        })
    }));

    tokio::spawn(post_process::summarize_search_results(
        settings.clone(),
        search_query_request,
        search_response.result,
        update_processor,
        openai_stream_regex,
        tx,
    ));

    let stream = ReceiverStream::new(rx).map(move |msg: api_models::SearchByIdResponse| {
        let json_data = serde_json::to_string(&msg).unwrap_or("".to_string());
        Ok(Event::default().data(json_data))
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(30))))
}

#[utoipa::path(
    get,
    path = "/search/one",
    responses((status = OK, body = SearchByIdResponse)),
    params(api_models::SearchByIdRequest)
)]
#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn get_one_search_result_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_by_id_request): Query<api_models::SearchByIdRequest>,
) -> crate::Result<Json<api_models::SearchByIdResponse>> {
    let search_history =
        services::get_one_search(&pool, &user.user_id, &search_by_id_request).await?;
    Ok(Json(search_history))
}

#[utoipa::path(
    get,
    path = "/search/history",
    responses((status = OK, body = ThreadHistoryResponse)),
    params(api_models::ThreadHistoryRequest)
)]
// (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
// (status = UNPROCESSABLE_ENTITY, description = "Invalid data", body = ErrorResponse),
// (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn get_threads_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(thread_history_request): Query<api_models::ThreadHistoryRequest>,
) -> crate::Result<Json<api_models::ThreadHistoryResponse>> {
    thread_history_request
        .validate()
        .map_err(|e| SearchError::InvalidData(format!("Invalid thread history request: {}", e)))?;

    let search_history =
        services::get_threads(&pool, &user.user_id, &thread_history_request).await?;
    Ok(Json(search_history))
}

#[utoipa::path(
    get,
    path = "/search/thread",
    responses((status = OK, body = SearchThreadResponse)),
    params(api_models::GetThreadRequest)
)]
#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn get_one_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(get_thread_request): Query<api_models::GetThreadRequest>,
) -> crate::Result<Json<api_models::SearchThreadResponse>> {
    get_thread_request
        .validate()
        .map_err(|e| SearchError::InvalidData(format!("Invalid get thread request: {}", e)))?;

    let search_thread = services::get_one_thread(&pool, &user.user_id, &get_thread_request).await?;
    Ok(Json(search_thread))
}

#[utoipa::path(
    patch,
    path = "/search/threads",
    responses((status = OK)),
    request_body = UpdateThreadRequest
)]
#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn update_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_thread_request): Json<api_models::UpdateThreadRequest>,
) -> crate::Result<()> {
    update_thread_request
        .validate()
        .map_err(|e| SearchError::InvalidData(format!("Invalid update thread request: {}", e)))?;

    services::update_thread(&pool, &user.user_id, &update_thread_request).await?;
    Ok(())
}

#[utoipa::path(
    patch,
    path = "/search/reaction",
    responses((status = OK)),
    request_body = SearchReactionRequest
)]
#[tracing::instrument(level = "info", skip_all, ret, err(Debug))]
async fn update_search_reaction_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(search_reaction_request): Json<api_models::SearchReactionRequest>,
) -> crate::Result<()> {
    services::update_search_reaction(&pool, &user.user_id, &search_reaction_request).await?;
    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_threads_handler,
        get_one_thread_handler,
        get_one_search_result_handler,
        update_thread_handler,
        update_search_reaction_handler,
    ),
    components(schemas(
        api_models::ThreadHistoryResponse,
        api_models::SearchThreadResponse,
        api_models::SearchByIdResponse,
        api_models::UpdateThreadRequest,
        api_models::SearchReactionRequest,
        data_models::Thread,
        data_models::Search,
        data_models::Source,
        data_models::SourceType,
    ))
)]
pub struct OpenApiDoc;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_query_handler))
        .route("/one", get(get_one_search_result_handler))
        .route("/threads", get(get_one_thread_handler))
        .route("/threads", patch(update_thread_handler))
        .route("/history", get(get_threads_handler))
        .route("/reaction", patch(update_search_reaction_handler))
}

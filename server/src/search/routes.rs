use crate::cache::CachePool;
use crate::proto::agency_service_client::AgencyServiceClient;
use crate::rag::{self, post_process, pre_process};
use crate::search::{api_models, services};
use crate::settings::Settings;
use crate::startup::AppState;
use crate::users::User;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use axum::routing::{get, patch};
use axum::{Json, Router};
use color_eyre::eyre::eyre;
use futures::stream::StreamExt;
use futures::Stream;
use regex::Regex;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_search_query_handler(
    State(settings): State<Settings>,
    State(brave_api_config): State<rag::BraveAPIConfig>,
    State(pool): State<PgPool>,
    State(cache): State<CachePool>,
    State(mut agency_service): State<AgencyServiceClient<Channel>>,
    State(openai_stream_regex): State<Regex>,
    user: User,
    Query(search_query_request): Query<api_models::SearchQueryRequest>,
) -> crate::Result<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let user_id = user.user_id;

    let (query_validity, rephrased_query) = tokio::join!(
        pre_process::check_query_validity(&settings, &search_query_request),
        pre_process::rephrase_query(&pool, &settings, &search_query_request)
    );

    if let Ok(false) = query_validity {
        return Err(eyre!("Query is invalid to proceed").into());
    }
    let rephrased_query = match rephrased_query {
        Ok(rephrased_query) => rephrased_query,
        _ => search_query_request.query.clone(),
    };

    let (search_item, search_response) = tokio::join!(
        services::insert_new_search(&pool, &user_id, &search_query_request, &rephrased_query),
        rag::search(
            &settings,
            &brave_api_config,
            &cache,
            &mut agency_service,
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
    .map_err(|e| eyre!("Failed to send end result: {}", e))?;

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

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_one_search_result_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(search_by_id_request): Query<api_models::SearchByIdRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history = services::get_one_search(&pool, &user_id, &search_by_id_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_threads_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(thread_history_request): Query<api_models::ThreadHistoryRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    let search_history = services::get_threads(&pool, &user_id, &thread_history_request).await?;

    Ok((StatusCode::OK, Json(search_history)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn get_one_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Query(get_thread_request): Query<api_models::GetThreadRequest>,
) -> crate::Result<impl IntoResponse> {
    let search_thread = services::get_one_thread(&pool, &user.user_id, &get_thread_request).await?;

    Ok((StatusCode::OK, Json(search_thread)))
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_thread_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(update_thread_request): Json<api_models::UpdateThreadRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::update_thread(&pool, &user_id, &update_thread_request).await?;

    Ok(StatusCode::OK)
}

#[tracing::instrument(level = "debug", skip_all, ret, err(Debug))]
async fn update_search_reaction_handler(
    State(pool): State<PgPool>,
    user: User,
    Json(search_reaction_request): Json<api_models::SearchReactionRequest>,
) -> crate::Result<impl IntoResponse> {
    let user_id = user.user_id;

    services::update_search_reaction(&pool, &user_id, &search_reaction_request).await?;

    Ok(StatusCode::OK)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_search_query_handler))
        .route("/one", get(get_one_search_result_handler))
        .route("/threads", get(get_one_thread_handler))
        .route("/threads", patch(update_thread_handler))
        .route("/history", get(get_threads_handler))
        .route("/reaction", patch(update_search_reaction_handler))
}

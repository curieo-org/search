use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::search::{get_search_history, get_top_searches, insert_search_history, search};
use server::search::{SearchHistoryRequest, SearchQueryRequest, SearchResponse, TopSearchRequest};
use server::settings::Settings;
use server::startup::cache_connect;
use server::Result;
use sqlx::PgPool;

#[sqlx::test]
async fn search_test() -> Result<()> {
    let settings = Settings::new();
    let mut connection = cache_connect(settings.cache.expose())
        .await
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let search_query = SearchQueryRequest {
        query: "test".to_string(),
    };

    let search_result = search(&mut connection, &search_query).await;

    assert!(search_result.is_ok());

    Ok(())
}

#[sqlx::test]
async fn top_searches_test() -> Result<()> {
    let settings = Settings::new();
    let mut connection = cache_connect(settings.cache.expose())
        .await
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let top_search_query = TopSearchRequest { limit: Some(1) };

    let top_searches_result = get_top_searches(&mut connection, &top_search_query).await;

    assert!(top_searches_result.is_ok());
    assert_eq!(top_searches_result.unwrap().len(), 1);

    Ok(())
}

#[sqlx::test]
async fn insert_search_and_get_search_history_test(pool: PgPool) -> Result<()> {
    let settings = Settings::new();
    let mut connection = cache_connect(settings.cache.expose())
        .await
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let new_user = register(
        pool.clone(),
        RegisterUserRequest {
            email: "test-email".to_string(),
            username: "test-username".to_string(),
            password: Some("password".to_string().into()),
            access_token: Default::default(),
        },
    )
    .await?;

    let user_id = new_user.user_id;
    let search_query = SearchQueryRequest {
        query: "test_query".to_string(),
    };
    let search_response = SearchResponse {
        result: "test_result".to_string(),
        sources: vec!["test_source".to_string()],
    };

    let search_insertion_result = insert_search_history(
        &pool,
        &mut connection,
        &user_id,
        &search_query,
        &search_response,
    )
    .await;

    assert!(search_insertion_result.is_ok());

    let search_history_request = SearchHistoryRequest {
        limit: Some(1),
        offset: Some(0),
    };

    let search_history_result = get_search_history(&pool, &user_id, &search_history_request).await;

    assert!(&search_history_result.is_ok());
    let search_history_result = search_history_result.unwrap();

    assert_eq!(&search_history_result.len(), &1);
    assert_eq!(&search_history_result[0].query, &search_query.query);
    assert_eq!(search_history_result[0].user_id, user_id);
    assert_eq!(search_history_result[0].result, search_response.result);
    assert_eq!(search_history_result[0].sources, search_response.sources);

    Ok(())
}

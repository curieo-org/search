use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::cache::{CachePool, CacheSettings};
use server::proto::{SearchResponse, Source};
use server::search::{
    get_search_history, get_top_searches, insert_search_history, search, update_search_reaction,
};
use server::search::{
    SearchHistoryRequest, SearchQueryRequest, SearchReactionRequest, TopSearchRequest,
};
use server::settings::Settings;
use server::startup::agency_service_connect;
use server::Result;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn search_test() -> Result<()> {
    let settings = Settings::new();
    let mut agency_service = agency_service_connect(&settings.agency_api.expose())
        .await
        .unwrap();
    let cache = CachePool::new(&settings.cache).await?;

    let search_query = SearchQueryRequest {
        session_id: Some(Uuid::new_v4()),
        query: "test".to_string(),
    };

    let search_result = search(&cache, &mut agency_service, &search_query).await;

    assert!(search_result.is_ok());
    assert_eq!(search_result.unwrap().status, 200);

    Ok(())
}

#[tokio::test]
async fn top_searches_test() -> Result<()> {
    let cache_settings = CacheSettings {
        url: "redis://127.0.0.1/".to_string().into(),
        enabled: true,
        ttl: 3600,
        max_sorted_size: 100,
    };
    let cache = CachePool::new(&cache_settings).await?;

    let top_search_query = TopSearchRequest { limit: Some(1) };

    let top_searches_result = get_top_searches(&cache, &top_search_query).await;
    assert!(top_searches_result.is_ok());
    assert_eq!(top_searches_result.unwrap().len(), 1);

    Ok(())
}

#[sqlx::test]
async fn insert_search_and_get_search_history_test(pool: PgPool) -> Result<()> {
    let settings = Settings::new();
    let cache = CachePool::new(&settings.cache).await?;

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
        session_id: Some(Uuid::new_v4()),
        query: "test_query".to_string(),
    };
    let search_response = SearchResponse {
        status: 200,
        result: "test_result".to_string(),
        sources: vec![Source {
            url: "test_url".to_string(),
            metadata: HashMap::from([
                ("test_key1".to_string(), "test_value1".to_string()),
                ("test_key2".to_string(), "test_value2".to_string()),
            ]),
        }],
    };

    let search_insertion_result =
        insert_search_history(&pool, &cache, &user_id, &search_query, &search_response).await;

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
    assert_eq!(search_history_result[0].sources.0, search_response.sources);

    Ok(())
}

#[sqlx::test]
async fn update_search_reaction_test(pool: PgPool) -> Result<()> {
    let settings = Settings::new();
    let cache = CachePool::new(&settings.cache).await?;

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
        session_id: None,
        query: "test_query".to_string(),
    };
    let search_response = SearchResponse {
        status: 200,
        result: "test_result".to_string(),
        sources: vec![Source {
            url: "test_url".to_string(),
            metadata: HashMap::from([
                ("test_key1".to_string(), "test_value1".to_string()),
                ("test_key2".to_string(), "test_value2".to_string()),
            ]),
        }],
    };

    let search_insertion_result =
        insert_search_history(&pool, &cache, &user_id, &search_query, &search_response).await;

    assert!(search_insertion_result.is_ok());
    let search_insertion_result = search_insertion_result.unwrap();

    let search_reaction_request = SearchReactionRequest {
        search_history_id: search_insertion_result.search_history_id,
        reaction: true,
    };

    let search_reaction_result =
        update_search_reaction(&pool, &user_id, &search_reaction_request).await;

    assert!(&search_reaction_result.is_ok());
    let search_reaction_result = search_reaction_result.unwrap();

    assert_eq!(&search_reaction_result.query, &search_query.query);
    assert_eq!(&search_reaction_result.user_id, &user_id);
    assert_eq!(&search_reaction_result.result, &search_response.result);
    assert_eq!(&search_reaction_result.sources.0, &search_response.sources);
    assert_eq!(
        search_reaction_result.reaction.unwrap(),
        search_reaction_request.reaction
    );

    Ok(())
}

use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::cache::CacheSettings;
use server::search::{
    get_search_history, get_top_searches, insert_search_history, search, update_search_reaction,
};
use server::search::{
    SearchHistoryRequest, SearchQueryRequest, SearchReactionRequest, SearchResponse,
    TopSearchRequest,
};
use server::settings::Settings;
use server::startup::cache_connect;
use server::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn search_test() -> Result<()> {
    let settings = Settings::new();
    let cache_settings = CacheSettings {
        cache_url: settings.cache_url.expose().to_string(),
        enabled: settings.cache_enabled,
        ttl: settings.cache_ttl,
        max_sorted_size: settings.cache_max_sorted_size,
    };
    let cache = cache_connect(&cache_settings).await?;

    let search_query = SearchQueryRequest {
        session_id: Some(Uuid::new_v4()),
        query: "test".to_string(),
    };

    let search_result = search(&cache, &search_query).await;

    assert!(search_result.is_ok());

    Ok(())
}

#[sqlx::test]
async fn top_searches_test() -> Result<()> {
    let settings = Settings::new();
    let cache_settings = CacheSettings {
        cache_url: settings.cache_url.expose().to_string(),
        enabled: settings.cache_enabled,
        ttl: settings.cache_ttl,
        max_sorted_size: settings.cache_max_sorted_size,
    };
    let cache = cache_connect(&cache_settings).await?;

    let top_search_query = TopSearchRequest { limit: Some(1) };

    let top_searches_result = get_top_searches(&cache, &top_search_query).await;

    assert!(top_searches_result.is_ok());
    assert_eq!(top_searches_result.unwrap().len(), 1);

    Ok(())
}

#[sqlx::test]
async fn insert_search_and_get_search_history_test(pool: PgPool) -> Result<()> {
    let settings = Settings::new();
    let cache_settings = CacheSettings {
        cache_url: settings.cache_url.expose().to_string(),
        enabled: settings.cache_enabled,
        ttl: settings.cache_ttl,
        max_sorted_size: settings.cache_max_sorted_size,
    };
    let cache = cache_connect(&cache_settings).await?;

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
        result: "test_result".to_string(),
        sources: vec!["test_source".to_string()],
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
    assert_eq!(search_history_result[0].sources, search_response.sources);

    Ok(())
}

#[sqlx::test]
async fn update_search_reaction_test(pool: PgPool) -> Result<()> {
    let settings = Settings::new();
    let cache_settings = CacheSettings {
        cache_url: settings.cache_url.expose().to_string(),
        enabled: settings.cache_enabled,
        ttl: settings.cache_ttl,
        max_sorted_size: settings.cache_max_sorted_size,
    };
    let cache = cache_connect(&cache_settings).await?;

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
        result: "test_result".to_string(),
        sources: vec!["test_source".to_string()],
    };

    let search_insertion_result =
        insert_search_history(&pool, &cache, &user_id, &search_query, &search_response).await;

    assert!(search_insertion_result.is_ok());
    let search_insertion_result = search_insertion_result.unwrap();

    let search_reaction_request = SearchReactionRequest {
        search_history_id: search_insertion_result.search_history_id.clone(),
        reaction: true,
    };

    let search_reaction_result =
        update_search_reaction(&pool, &user_id, &search_reaction_request).await;

    assert!(&search_reaction_result.is_ok());
    let search_reaction_result = search_reaction_result.unwrap();

    assert_eq!(&search_reaction_result.query, &search_query.query);
    assert_eq!(&search_reaction_result.user_id, &user_id);
    assert_eq!(&search_reaction_result.result, &search_response.result);
    assert_eq!(&search_reaction_result.sources, &search_response.sources);
    assert_eq!(
        search_reaction_result.reaction.unwrap(),
        search_reaction_request.reaction
    );

    Ok(())
}

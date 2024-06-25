use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::cache::{CachePool, CacheSettings};
use server::rag::{search, SearchResponse, Source};
use server::search::{
    get_one_search, insert_new_search, update_search_reaction, SearchByIdRequest, SourceType,
};
use server::search::{SearchQueryRequest, SearchReactionRequest};
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
    let brave_api_config = settings.brave.clone().into();

    let search_result = search(
        &settings,
        &brave_api_config,
        &cache,
        &mut agency_service,
        "test",
    )
    .await;

    assert!(search_result.is_ok());
    assert_eq!(search_result.unwrap().result, "");

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
        thread_id: Some(Uuid::new_v4()),
        query: "test-query".to_string(),
    };
    let rephrased_query = "test-rephrased-query";
    let expected_response = SearchResponse {
        result: "test-result".to_string(),
        sources: vec![Source {
            url: "test-url".to_string(),
            title: "test-title".to_string(),
            description: "test-description".to_string(),
            source_type: SourceType::Pdf,
            metadata: HashMap::from([
                ("test_key1".to_string(), "test_value1".to_string()),
                ("test_key2".to_string(), "test_value2".to_string()),
            ]),
        }],
    };

    let search_insertion_result =
        insert_new_search(&pool, &user_id, &search_query, rephrased_query).await?;

    let one_search_history_request = SearchByIdRequest {
        search_id: search_insertion_result.search_id,
    };

    let actual_response = get_one_search(&pool, &user_id, &one_search_history_request).await?;

    assert_eq!(actual_response.search.query, search_query.query);
    assert_eq!(actual_response.search.result, expected_response.result);
    assert_eq!(actual_response.sources.len(), 1);
    let actual_source = actual_response.sources[0].clone();
    let expected_source = expected_response.sources[0].clone();
    assert_eq!(actual_source.title, expected_source.title);

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
        thread_id: Some(Uuid::new_v4()),
        query: "test-query".to_string(),
    };
    let rephrased_query = "test-rephrased-query";
    let expected_response = SearchResponse {
        result: "test-result".to_string(),
        sources: vec![Source {
            url: "test-url".to_string(),
            title: "test-title".to_string(),
            description: "test-description".to_string(),
            source_type: SourceType::Pdf,
            metadata: HashMap::from([
                ("test_key1".to_string(), "test_value1".to_string()),
                ("test_key2".to_string(), "test_value2".to_string()),
            ]),
        }],
    };
    let search_insertion_result =
        insert_new_search(&pool, &user_id, &search_query, rephrased_query).await?;

    let search_reaction_request = SearchReactionRequest {
        search_id: search_insertion_result.search_id,
        reaction: true,
    };

    let search_reaction_result =
        update_search_reaction(&pool, &user_id, &search_reaction_request).await?;

    assert_eq!(&search_reaction_result.query, &search_query.query);
    assert_eq!(&search_reaction_result.result, &expected_response.result);
    assert_eq!(
        search_reaction_result.reaction.unwrap(),
        search_reaction_request.reaction
    );

    Ok(())
}

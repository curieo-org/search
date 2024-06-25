use server::auth::models::RegisterUserRequest;
use server::auth::register;
use server::cache::CachePool;
use server::rag::{search, SearchResponse, Source};
use server::search::{
    get_one_search, get_threads, insert_new_search, SearchByIdRequest, ThreadHistoryRequest,
};
use server::search::{SearchQueryRequest, SourceType};
use server::settings::Settings;
use server::startup::agency_service_connect;
use server::Result;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn search_test() -> Result<()> {
    let settings = Settings::new();
    let mut agency_service = agency_service_connect(settings.agency_api.expose())
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
    let search_response = SearchResponse {
        result: "test-result".to_string(),
        sources: vec![Source {
            url: "test-url".to_string(),
            title: "test-title".to_string(),
            description: "test-description".to_string(),
            source_type: SourceType::from(0),
            metadata: HashMap::from([
                ("test-key1".to_string(), "test-value1".to_string()),
                ("test-key2".to_string(), "test-value2".to_string()),
            ]),
        }],
    };

    let search_insertion_result = insert_new_search(&pool, &user_id, &search_query, "").await;

    assert!(search_insertion_result.is_ok());

    let one_search_history_request = SearchByIdRequest {
        search_id: search_insertion_result.unwrap().search_id,
    };

    let one_search_history_result =
        get_one_search(&pool, &user_id, &one_search_history_request).await;

    assert!(one_search_history_result.is_ok());
    let one_search_history_result = one_search_history_result.unwrap();

    assert_eq!(one_search_history_result.search.query, search_query.query);
    assert_eq!(
        one_search_history_result.search.result,
        search_response.result
    );
    for (a, b) in one_search_history_result
        .sources
        .iter()
        .zip(search_response.sources)
    {
        assert_eq!(a.url, b.url);
    }

    let search_history_request = ThreadHistoryRequest {
        limit: Some(1),
        offset: Some(0),
    };

    let search_history_result = get_threads(&pool, &user_id, &search_history_request).await;

    assert!(&search_history_result.is_ok());
    let search_history_result = search_history_result.unwrap().threads;

    assert_eq!(&search_history_result.len(), &1);
    assert_eq!(search_history_result[0].user_id, user_id);

    Ok(())
}

use httpmock::prelude::POST;
use httpmock::MockServer;
use server::auth::{register, RegisterUserRequest};
use server::cache::CachePool;
use server::llms::{PromptCompressionAPIResponse, PromptCompressionOutput};
use server::rag::search;
use server::search::{
    append_search_result, get_one_search, insert_new_search, update_search_reaction,
    SearchByIdRequest,
};
use server::search::{SearchQueryRequest, SearchReactionRequest};
use server::settings::Settings;
use server::Result;
use sqlx::PgPool;

mod utils;

#[tokio::test]
async fn search_test() -> Result<()> {
    let mut settings = Settings::new();

    let (server_future, mut agency_service) = utils::agency_server_and_client_stub().await;
    let cache = CachePool::new(&settings.cache).await?;
    let brave_api_config = settings.brave.clone().into();

    // Mock compression server
    let server = MockServer::start();
    settings.llm.prompt_compression_url = server.url("/compress");
    let compression_response = PromptCompressionAPIResponse {
        response: PromptCompressionOutput {
            compressed_prompt: "test-compressed-prompt".to_string(),
        },
    };
    let _ = server.mock(|when, then| {
        when.method(POST).path("/compress");

        then.status(200)
            .header("content-type", "application/json")
            .json_body_obj(&compression_response);
    });

    let request_future = async {
        let search_result = search(
            &settings,
            &brave_api_config,
            &cache,
            &mut agency_service,
            "test",
        )
        .await;
        // Validate server response with assertions
        assert_eq!(search_result.unwrap().result, "test-compressed-prompt");
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = server_future => panic!("server returned first"),
        _ = request_future => (),
    }

    Ok(())
}

#[sqlx::test]
async fn insert_search_and_get_search_history_test(pool: PgPool) -> Result<()> {
    let new_user = register(
        pool.clone(),
        RegisterUserRequest {
            email: "test-email".to_string(),
            password: Some("password".to_string().into()),
            access_token: Default::default(),
        },
    )
    .await?;

    let user_id = new_user.user_id;
    let search_query = SearchQueryRequest {
        thread_id: None,
        query: "test-query".to_string(),
    };
    let rephrased_query = "test-rephrased-query";

    let search_result = insert_new_search(&pool, &user_id, &search_query, rephrased_query).await?;
    let search_id = search_result.search_id;
    let one_search_history_request = SearchByIdRequest { search_id };

    let actual_response = get_one_search(&pool, &user_id, &one_search_history_request).await?;
    assert_eq!(actual_response.search.query, search_query.query);
    assert_eq!(actual_response.sources.len(), 0);

    // update the search result
    append_search_result(&pool, &search_result, "updated-result").await?;

    let updated_response = get_one_search(&pool, &user_id, &one_search_history_request).await?;
    assert_eq!(updated_response.search.query, search_query.query);
    assert_eq!(updated_response.search.result, "updated-result");

    Ok(())
}

#[sqlx::test]
async fn update_search_reaction_test(pool: PgPool) -> Result<()> {
    let new_user = register(
        pool.clone(),
        RegisterUserRequest {
            email: "test-email".to_string(),
            password: Some("password".to_string().into()),
            access_token: Default::default(),
        },
    )
    .await?;

    let user_id = new_user.user_id;
    let search_query = SearchQueryRequest {
        thread_id: None,
        query: "test-query".to_string(),
    };
    let rephrased_query = "test-rephrased-query";
    let search_result = insert_new_search(&pool, &user_id, &search_query, rephrased_query).await?;
    let search_id = search_result.search_id;

    let search_reaction_request = SearchReactionRequest {
        search_id,
        reaction: true,
    };

    let search_reaction_result =
        update_search_reaction(&pool, &user_id, &search_reaction_request).await?;

    assert_eq!(&search_reaction_result.query, &search_query.query);
    assert_eq!(search_reaction_result.reaction, Some(true));

    Ok(())
}

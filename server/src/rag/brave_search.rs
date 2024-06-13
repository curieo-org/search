use crate::rag::{RetrievedResult, Source};
use crate::search::SourceType;
use crate::secrets::Secret;
use color_eyre::eyre::eyre;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraveSettings {
    pub url: String,
    pub goggles_id: String,
    pub subscription_key: Secret<String>,
    pub count: u16,
    pub result_filter: String,
    pub search_lang: String,
    pub extra_snippets: bool,
    pub safesearch: String,
}

#[derive(Debug, Clone)]
pub struct BraveAPIConfig {
    pub queries: Vec<(String, String)>,
    pub headers: HeaderMap<HeaderValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraveWebSearchResult {
    pub title: String,
    pub url: String,
    pub description: String,
    pub page_age: Option<String>,
    pub age: Option<String>,
    pub language: Option<String>,
    pub extra_snippets: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BraveWebAPIResponse {
    pub results: Vec<BraveWebSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BraveAPIResponse {
    pub web: BraveWebAPIResponse,
}

pub fn prepare_brave_api_config(brave_settings: &BraveSettings) -> BraveAPIConfig {
    let queries = vec![
        (String::from("count"), brave_settings.count.to_string()),
        (
            String::from("goggles_id"),
            brave_settings.goggles_id.clone(),
        ),
        (
            String::from("result_filter"),
            brave_settings.result_filter.clone(),
        ),
        (
            String::from("search_lang"),
            brave_settings.search_lang.clone(),
        ),
        (
            String::from("extra_snippets"),
            brave_settings.extra_snippets.to_string(),
        ),
        (
            String::from("safesearch"),
            brave_settings.safesearch.clone(),
        ),
    ];

    let headers = HeaderMap::from_iter(
        vec![
            ("Accept", "application/json"),
            ("Accept-Encoding", "gzip"),
            (
                "X-Subscription-Token",
                brave_settings.subscription_key.expose(),
            ),
        ]
        .into_iter()
        .map(|(k, v)| {
            (
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            )
        }),
    );

    BraveAPIConfig { queries, headers }
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn web_search(
    brave_settings: &BraveSettings,
    brave_api_config: &BraveAPIConfig,
    search_query: &str,
) -> crate::Result<Vec<RetrievedResult>> {
    let api_url = brave_settings.url.clone() + "?q=" + search_query;

    let client = Client::new();
    let response = client
        .get(&api_url)
        .query(&brave_api_config.queries)
        .headers(brave_api_config.headers.clone())
        .send()
        .await
        .map_err(|e| eyre!("Request to brave failed: {e}"))?;

    if !response.status().is_success() {
        response
            .error_for_status_ref()
            .map_err(|e| eyre!("Request failed: {e}"))?;
    }

    let brave_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| eyre!("Failed to parse response: {e}"))?;
    let brave_response: BraveAPIResponse = serde_json::from_value(brave_response)
        .map_err(|e| eyre!("Failed to parse response: {e}"))?;

    let retrieved_results: Vec<RetrievedResult> = brave_response
        .web
        .results
        .into_iter()
        .map(convert_to_retrieved_result)
        .collect();

    Ok(retrieved_results)
}

fn convert_to_retrieved_result(result: BraveWebSearchResult) -> RetrievedResult {
    let extra_snippets = match result.extra_snippets {
        Some(snippets) => snippets,
        None => vec![],
    };

    RetrievedResult {
        text: result.description.clone() + "\n\n" + extra_snippets.join("\n\n").as_str(),
        source: Source {
            title: result.title,
            url: result.url,
            description: result.description,
            source_type: SourceType::Url,
            metadata: HashMap::from_iter(
                vec![
                    (
                        "page_age".to_string(),
                        result.page_age.unwrap_or("".to_string()),
                    ),
                    ("age".to_string(), result.age.unwrap_or("".to_string())),
                    (
                        "language".to_string(),
                        result.language.unwrap_or("".to_string()),
                    ),
                ]
                .into_iter(),
            ),
        },
    }
}

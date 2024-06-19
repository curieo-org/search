use crate::secrets::Secret;
use color_eyre::eyre::eyre;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRephraserSettings {
    pub api_key: Secret<String>,
    pub api_url: String,
    pub max_tokens: u16,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub query: String,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRephraserInput {
    pub query: String,
    pub previous_context: Vec<QueryResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryRephraserAPIResponse {
    pub output: Output,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRephraserOutput {
    pub rephrased_query: String,
}

fn prepare_prompt(query_rephraser_input: &QueryRephraserInput) -> String {
    "Rephrase the input text based on the context and the final sentence. So that it can be understood without the context.\n\n---\n\nFollow the following format.\n\nContext: contains the chat history\n\nQuestion: ${question}\n\nReasoning: Let's think step by step in order to ${produce the answer}. We ...\n\nAnswer: Given a chat history and the latest user question, which might reference the context from the chat history, formulate a standalone question that can be understood from the history without needing the chat history. DO NOT ANSWER THE QUESTION - just reformulate it\n\n---\n\nContext: ".to_string()
        + query_rephraser_input.previous_context.iter().map(|x| format!("{}: {}", x.query, x.result)).collect::<Vec<String>>().join("\n").as_str()
        + "\n\nQuestion: "
        + query_rephraser_input.query.as_str()
        + "\n\nReasoning: Let's think step by step in order to...\n\nAnswer: "
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rephrase_query(
    settings: &QueryRephraserSettings,
    query_rephraser_input: &QueryRephraserInput,
) -> crate::Result<QueryRephraserOutput> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_bytes(b"Authorization")
            .map_err(|e| eyre!("Failed to create header: {e}"))?,
        HeaderValue::from_str(&settings.api_key.expose())
            .map_err(|e| eyre!("Failed to create header: {e}"))?,
    );

    let prompt = prepare_prompt(query_rephraser_input);

    let response = client
        .post(&settings.api_url)
        .json(&serde_json::json!({
            "model": settings.model,
            "prompt": prompt,
            "max_tokens": settings.max_tokens,
            // "temperature": 1.0,
            // "top_p": 1.0,
            // "top_k": 50,
            // "frequency_penalty": 0.0,
            // "presence_penalty": 0.0,
            // "repetition_penalty": 1.0,
        }))
        .headers(headers)
        .send()
        .await
        .map_err(|e| eyre!("Request to query rephraser failed: {e}"))?;

    let response_body = serde_json::from_slice::<QueryRephraserAPIResponse>(
        &response
            .bytes()
            .await
            .map_err(|e| eyre!("Failed to read response: {e}"))?,
    )
    .map_err(|e| eyre!("Failed to parse response: {e}"))?;

    Ok(QueryRephraserOutput {
        rephrased_query: response_body.output.choices[0].text.trim().to_string(),
    })
}

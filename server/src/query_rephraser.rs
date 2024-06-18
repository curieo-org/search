use crate::llms::LLMSettings;
use color_eyre::eyre::eyre;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rephrase_query(
    llm_settings: &LLMSettings,
    query_rephraser_input: &QueryRephraserInput,
) -> crate::Result<QueryRephraserOutput> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_bytes(b"Authorization")
            .map_err(|e| eyre!("Failed to create header: {e}"))?,
        HeaderValue::from_str(
            "Bearer 92b2f1a8c6f71fdaa546324a453fcd356c112a27dac140ecb1ef090be7a326ae",
        )
        .map_err(|e| eyre!("Failed to create header: {e}"))?,
    );

    let prompt = format!(
        "Rephrase questions in such a way that history is not needed.

        \n\n---\n\nFollow the following format.

        \n\nContext: {}

        \n\nQuestion: {}

        \n\nReasoning: Let's think step by step in order to produce the answer. We ...

        \n\nAnswer: Given a chat history and the latest user question, which might reference the context from the chat
        history, formulate a standalone question that can be understood from the history without needing the chat history.

        \n\n------",
        query_rephraser_input.previous_context.iter().map(|x| format!("{}: {}", x.query, x.result)).collect::<Vec<String>>().join("\n"),
        query_rephraser_input.query
    );
    let response = client
        .post("https://api.together.xyz/inference")
        .json(&serde_json::json!({
            "model": "meta-llama/Llama-2-7b-hf",
            "prompt": prompt,
            "max_tokens": 50,
            "temperature": 1.0,
            "top_p": 1.0,
            "top_k": 50,
            // "frequency_penalty": 0.0,
            // "presence_penalty": 0.0,
            "repetition_penalty": 1.0,
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
    println!("Rephrased query: {}", response_body.output.choices[0].text);

    Ok(QueryRephraserOutput {
        rephrased_query: query_rephraser_input.query.clone(),
    })
}

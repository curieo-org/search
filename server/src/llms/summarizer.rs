use crate::llms::OpenAISettings;
use crate::search::api_models;
use color_eyre::eyre::eyre;
use futures::StreamExt;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizerSettings {
    pub api_url: String,
    pub model: String,
    pub max_new_tokens: u16,
    pub temperature: f32,
    pub top_p: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummarizerParams {
    pub model: Option<String>,
    pub max_new_tokens: Option<u16>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummarizerAPIInput {
    pub inputs: String,
    pub parameters: SummarizerParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizerInput {
    pub query: String,
    pub retrieved_result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub special: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizerStreamOutput {
    pub token: Token,
    pub generated_text: Option<String>,
}

fn prepare_llm_context_string(
    settings: &SummarizerSettings,
    summarizer_input: SummarizerInput,
) -> SummarizerAPIInput {
    SummarizerAPIInput {
        inputs: format!("In this exercise you will assume the role of a scientific medical assistant. Your task is to answer the provided question as best as you can, based on the provided solution draft.
        The solution draft follows the format \"Thought, Action, Action Input, Observation\", where the 'Thought' statements describe a reasoning sequence. The rest of the text is information obtained to complement the reasoning sequence, and it is 100% accurate OR you can use a single \"Final Answer\" format.
        Your task is to write an answer to the question based on the solution draft, and the following guidelines:
        The text should have an educative and assistant-like tone, be accurate, follow the same reasoning sequence than the solution draft and explain how any conclusion is reached.
        Question: {}\n\nSolution draft: {}\n\nAnswer:", summarizer_input.query, summarizer_input.retrieved_result),
        parameters: SummarizerParams {
            model: Some(settings.model.clone()),
            max_new_tokens: Some(settings.max_new_tokens.clone()),
            temperature: Some(settings.temperature),
            top_p: Some(settings.top_p),
        },
    }
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn generate_text_with_llm(
    settings: SummarizerSettings,
    summarizer_input: SummarizerInput,
    update_processor: api_models::UpdateResultProcessor,
    tx: Sender<api_models::SearchByIdResponse>,
) -> crate::Result<()> {
    let summarizer_input = prepare_llm_context_string(&settings, summarizer_input);
    let client = Client::new();

    let response = client
        .post(settings.api_url.as_str())
        .json(&summarizer_input)
        .send()
        .await
        .map_err(|e| eyre!("Request to summarizer failed: {e}"))?;

    // stream the response
    if !response.status().is_success() {
        return Err(eyre!("Request failed with status: {:?}", response.status()).into());
    }
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        // remove `data` from the start of the chunk and `\n\n` from the end
        let chunk = chunk.map_err(|e| eyre!("Failed to read chunk: {e}"))?;
        let chunk = &chunk[5..chunk.len() - 2];

        let summarizer_api_response = serde_json::from_slice::<SummarizerStreamOutput>(&chunk)
            .map_err(|e| eyre!("Failed to parse summarizer response: {e}"))?;

        if !summarizer_api_response.token.special {
            let mut search = update_processor
                .process(summarizer_api_response.token.text.clone())
                .await
                .map_err(|e| eyre!("Failed to update result: {e}"))?;

            buffer.push_str(&summarizer_api_response.token.text);
            search.result = buffer.clone();
            let tx_response = tx
                .send(api_models::SearchByIdResponse {
                    search,
                    sources: vec![],
                })
                .await;

            if let Ok(_) = tx_response {
                buffer.clear();
            }
        }
    }

    Ok(())
}

fn prepare_openai_input(
    settings: &OpenAISettings,
    summarizer_input: SummarizerInput,
) -> serde_json::Value {
    let system_role = "You are a summarizer AI. In this exercise you will assume the role of a scientific medical assistant. Your task is to answer the provided question as best as you can, based on the provided solution draft.
    The solution draft follows the format \"Thought, Action, Action Input, Observation\", where the 'Thought' statements describe a reasoning sequence. The rest of the text is information obtained to complement the reasoning sequence, and it is 100% accurate OR you can use a single \"Final Answer\" format.
    Your task is to write an answer to the question based on the solution draft, and the following guidelines:
    The text should have an educative and assistant-like tone, be accurate, follow the same reasoning sequence than the solution draft and explain how any conclusion is reached.
    Question: {}\n\nSolution draft: {}\n\nAnswer: ";

    let user_input = format!(
        "Question: {}\n\nSolution draft: {}",
        summarizer_input.query, summarizer_input.retrieved_result
    );

    serde_json::json!({
        "model": settings.model,
        "stream": true,
        "messages": [
            {
                "role": "system",
                "content": system_role
            },
            {
                "role": "user",
                "content": user_input
            }
        ]
    })
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn generate_text_with_openai(
    settings: OpenAISettings,
    summarizer_input: SummarizerInput,
    update_processor: api_models::UpdateResultProcessor,
    stream_regex: Regex,
    tx: Sender<api_models::SearchByIdResponse>,
) -> crate::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_bytes(b"Authorization")
            .map_err(|e| eyre!("Failed to create header: {e}"))?,
        HeaderValue::from_str(settings.api_key.expose())
            .map_err(|e| eyre!("Failed to create header: {e}"))?,
    );

    let client = Client::new();
    let summarizer_input = prepare_openai_input(&settings, summarizer_input);

    let response = client
        .post(settings.api_url.as_str())
        .json(&summarizer_input)
        .headers(headers)
        .send()
        .await
        .map_err(|e| eyre!("Request to summarizer failed: {e}"))?;

    // stream the response
    if !response.status().is_success() {
        return Err(eyre!("Request failed with status: {:?}", response.status()).into());
    }

    let mut stream = response.bytes_stream();
    let mut stream_data = String::new();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| eyre!("Failed to read chunk: {e}"))?;
        stream_data.push_str(&String::from_utf8_lossy(&chunk));

        let parsed_chunk = stream_regex
            .captures_iter(&stream_data)
            .map(|c| c[1].to_string())
            .collect::<Vec<String>>()
            .join("");

        let last_index = stream_regex
            .captures_iter(&stream_data)
            .last()
            .map(|c| {
                if let Some(m) = c.get(0) {
                    return Some(m.end());
                }
                None
            })
            .unwrap_or(None);

        if let Some(last_index) = last_index {
            stream_data = stream_data.split_off(last_index);
        }

        let mut search = update_processor
            .process(parsed_chunk.clone())
            .await
            .map_err(|e| eyre!("Failed to update result: {e}"))?;

        buffer.push_str(&parsed_chunk);
        search.result = buffer.clone();
        let tx_response = tx
            .send(api_models::SearchByIdResponse {
                search,
                sources: vec![],
            })
            .await;

        if let Ok(_) = tx_response {
            buffer.clear();
        }
    }

    Ok(())
}

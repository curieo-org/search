use crate::err::AppError;
use crate::search::api_models;
use futures::StreamExt;
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

fn prepare_context_string(
    settings: &SummarizerSettings,
    summarizer_input: SummarizerInput,
) -> SummarizerAPIInput {
    SummarizerAPIInput {
        inputs: format!("In this exercise you will assume the role of a scientific medical assistant. Your task is to answer the provided question as best as you can, based on the provided solution draft.
        The solution draft follows the format \"Thought, Action, Action Input, Observation\", where the 'Thought' statements describe a reasoning sequence. The rest of the text is information obtained to complement the reasoning sequence, and it is 100% accurate OR you can use a single \"Final Answer\" format.
        Your task is to write an answer to the question based on the solution draft, and the following guidelines:
        The text should have an educative and assistant-like tone, be accurate, follow the same reasoning sequence than the solution draft and explain how any conclusion is reached.
        Question: {}\n\nSolution draft: {}\n\nAnswer:", summarizer_input.retrieved_result, summarizer_input.query),
        parameters: SummarizerParams {
            model: Some(settings.model.clone()),
            max_new_tokens: Some(settings.max_new_tokens.clone()),
            temperature: Some(settings.temperature),
            top_p: Some(settings.top_p),
        },
    }
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn generate_text_stream(
    settings: SummarizerSettings,
    summarizer_input: SummarizerInput,
    update_processor: api_models::UpdateResultProcessor,
    tx: Sender<api_models::SearchByIdResponse>,
) -> crate::Result<()> {
    let summarizer_input = prepare_context_string(&settings, summarizer_input);
    let client = Client::new();

    let response = client
        .post(settings.api_url.as_str())
        .json(&summarizer_input)
        .send()
        .await
        .map_err(|e| {
            AppError::ServiceUnavailable(format!("Request to summarizer failed: {}", e))
        })?;

    // stream the response
    if !response.status().is_success() {
        return Err(AppError::NotFound(format!(
            "Request failed with status: {:?}",
            response.status()
        )));
    }
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        // remove `data` from the start of the chunk and `\n\n` from the end
        let chunk =
            chunk.map_err(|e| AppError::InvalidResponse(format!("Failed to read chunk: {e}")))?;
        let chunk = &chunk[5..chunk.len() - 2];

        let summarizer_api_response = serde_json::from_slice::<SummarizerStreamOutput>(&chunk)
            .map_err(|e| {
                AppError::InvalidResponse(format!("Failed to parse summarizer response: {e}"))
            })?;

        if !summarizer_api_response.token.special {
            let mut search = update_processor
                .process(summarizer_api_response.token.text.clone())
                .await?;
            search.result = summarizer_api_response.token.text;

            tx.send(api_models::SearchByIdResponse {
                search,
                sources: vec![],
            })
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to send response: {e}")))?;
        }
    }

    Ok(())
}

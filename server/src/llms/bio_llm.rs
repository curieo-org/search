use color_eyre::eyre::eyre;
use futures::StreamExt;
use crate::llms::LLMSettings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use crate::rag::SearchResponse;
use crate::search::api_models;

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMParams {
    pub model: Option<String>,
    pub max_new_tokens: Option<u16>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BioLLMAPIInput {
    pub inputs: String,
    pub parameters: BioLLMParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMInput {
    pub query: String,
    pub retrieved_result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub special: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMStreamOutput {
    pub token: Token,
    pub generated_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMOutput {
    pub generated_text: String,
}

fn prepare_context_string(bio_llm_input: BioLLMInput) -> BioLLMAPIInput {
    BioLLMAPIInput {
        inputs: format!("In this exercise you will assume the role of a scientific medical assistant. Your task is to answer the provided question as best as you can, based on the provided solution draft.
        The solution draft follows the format \"Thought, Action, Action Input, Observation\", where the 'Thought' statements describe a reasoning sequence. The rest of the text is information obtained to complement the reasoning sequence, and it is 100% accurate OR you can use a single \"Final Answer\" format.
        Your task is to write an answer to the question based on the solution draft, and the following guidelines:
        The text should have an educative and assistant-like tone, be accurate, follow the same reasoning sequence than the solution draft and explain how any conclusion is reached.
        Question: {}

        Solution draft: {}

        Answer:", bio_llm_input.retrieved_result, bio_llm_input.query),
            parameters: BioLLMParams {
            model: None,
            max_new_tokens: Some(1024),
            temperature: Some(1.0),
            top_p: Some(0.7),
        },
    }
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn generate_text(
    llm_settings: &LLMSettings,
    bio_llm_input: BioLLMInput,
) -> crate::Result<BioLLMOutput> {
    let bio_llm_input = prepare_context_string(bio_llm_input);
    let client = Client::new();

    let response = client
        .post(llm_settings.bio_llm_url.as_str())
        .json(&bio_llm_input)
        .send()
        .await
        .map_err(|e| eyre!("Request to bio_llm failed: {e}"))?;

    let bio_llm_api_response = response
        .json::<BioLLMOutput>()
        .await
        .map_err(|e| eyre!("Failed to parse bio_llm response: {e}"))?;
    Ok(bio_llm_api_response)
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn generate_text_stream(
    llm_settings: LLMSettings,
    bio_llm_input: BioLLMInput,
    update_processor: api_models::UpdateResultProcessor,
    tx: Sender<SearchResponse>,
) -> crate::Result<()> {
    let bio_llm_input = prepare_context_string(bio_llm_input);
    let client = Client::new();

    let response = client
        .post(llm_settings.bio_llm_url.as_str())
        .json(&bio_llm_input)
        .send()
        .await
        .map_err(|e| eyre!("Request to bio_llm failed: {e}"))?;

    // stream the response
    if !response.status().is_success() {
        return Err(eyre!("Request failed with status: {:?}", response.status()).into());
    }
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        // remove `data` from the start of the chunk and `\n\n` from the end
        let chunk = chunk.map_err(|e| eyre!("Failed to read chunk: {e}"))?;
        let chunk = &chunk[5..chunk.len() - 2];

        let bio_llm_api_response = serde_json::from_slice::<BioLLMStreamOutput>(&chunk)
            .map_err(|e| eyre!("Failed to parse bio_llm response: {e}"))?;

        if !bio_llm_api_response.token.special {
            update_processor.process(bio_llm_api_response.token.text.clone())
                .await
                .map_err(|e| eyre!("Failed to update result: {e}"))?;

            tx.send(SearchResponse {
                result: bio_llm_api_response.token.text,
                sources: vec![],
            })
            .await
            .map_err(|e| eyre!("Failed to send response: {e}"))?;
        }
    }

    Ok(())
}

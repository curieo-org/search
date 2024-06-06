use crate::llms::LLMSettings;
use color_eyre::eyre::eyre;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmLinguaInput {
    pub query: String,
    pub target_token: u16,
    pub context_texts_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmLinguaOutput {
    pub compressed_prompt: String,
    pub compressed_prompt_list: Vec<String>,
    pub origin_tokens: u16,
    pub compressed_tokens: u16,
    pub ratio: String,
    pub sources: Vec<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmLinguaAPIResponse {
    pub response: LlmLinguaOutput,
}

pub async fn compress_and_rerank(
    llm_settings: &LLMSettings,
    llm_lingua_input: LlmLinguaInput,
) -> crate::Result<LlmLinguaOutput> {
    let client = Client::new();
    let response = client
        .post(llm_settings.llm_lingua_url.as_str())
        .json(&llm_lingua_input)
        .send()
        .await
        .map_err(|e| eyre!("Request to llm_lingua failed: {e}"))?;

    let llm_lingua_api_response = response
        .json::<LlmLinguaAPIResponse>()
        .await
        .map_err(|e| eyre!("Failed to parse llm_lingua response: {e}"))?;

    Ok(llm_lingua_api_response.response)
}

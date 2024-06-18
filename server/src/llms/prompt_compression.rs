use crate::err::AppError;
use crate::llms::LLMSettings;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompressionInput {
    pub query: String,
    pub target_token: u16,
    pub context_texts_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCompressionOutput {
    pub compressed_prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PromptCompressionAPIResponse {
    pub response: PromptCompressionOutput,
}

pub async fn compress(
    llm_settings: &LLMSettings,
    prompt_compression_input: PromptCompressionInput,
) -> crate::Result<PromptCompressionOutput> {
    let client = Client::new();
    let response = client
        .post(llm_settings.prompt_compression_url.as_str())
        .json(&prompt_compression_input)
        .send()
        .await
        .map_err(|e| {
            AppError::ServiceUnavailable(format!("Request to prompt compression failed: {}", e))
        })?;

    let prompt_compression_response = response
        .json::<PromptCompressionAPIResponse>()
        .await
        .map_err(|e| {
            AppError::InvalidResponse(format!(
                "Failed to parse prompt compression response: {}",
                e
            ))
        })?;

    Ok(prompt_compression_response.response)
}

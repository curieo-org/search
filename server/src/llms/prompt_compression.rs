use crate::llms::LLMSettings;
use crate::search::SearchError;
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
pub struct PromptCompressionAPIResponse {
    pub response: PromptCompressionOutput,
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn compress(
    llm_settings: &LLMSettings,
    prompt_compression_input: PromptCompressionInput,
) -> Result<PromptCompressionOutput, SearchError> {
    let client = Client::new();
    let response = client
        .post(llm_settings.prompt_compression_url.as_str())
        .json(&prompt_compression_input)
        .send()
        .await?;

    let prompt_compression_response = response.json::<PromptCompressionAPIResponse>().await?;

    Ok(prompt_compression_response.response)
}

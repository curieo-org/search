use crate::llms::LLMSettings;
use color_eyre::eyre::eyre;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToxicityInput {
    pub inputs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToxicityAPIResponse (pub Vec<ToxicityScore>);

#[derive(Debug, Serialize, Deserialize)]
struct ToxicityScore {
    pub score: f64,
    pub label: String,
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn predict_toxicity(
    llm_settings: &LLMSettings,
    toxicity_input: ToxicityInput,
) -> crate::Result<bool> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_bytes(b"Authorization").map_err(|e| eyre!("Failed to create header: {e}"))?,
        HeaderValue::from_str(&llm_settings.toxicity_auth_token.expose()).map_err(|e| eyre!("Failed to create header: {e}"))?,
    );
    let client = Client::new();

    let response = client
        .post(llm_settings.toxicity_url.as_str())
        .json(&toxicity_input)
        .headers(headers)
        .send()
        .await
        .map_err(|e| eyre!("Request to toxicity failed: {e}"))?;

    let toxicity_api_response: Vec<ToxicityScore> = response
        .json()
        .await
        .map_err(|e| eyre!("Failed to parse toxicity response: {e}"))?;

    let toxicity_score = toxicity_api_response.into_iter().find(|x| x.label == String::from("toxic")).unwrap_or(ToxicityScore {
        score: 0.0,
        label: String::from(""),
    });
    Ok(toxicity_score.score > llm_settings.toxicity_threshold)
}

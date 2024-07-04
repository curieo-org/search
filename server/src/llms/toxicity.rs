use crate::llms::LLMSettings;
use crate::search::SearchError;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToxicityInput {
    pub inputs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToxicityAPIResponse(pub Vec<ToxicityScore>);

#[derive(Debug, Serialize, Deserialize)]
struct ToxicityScore {
    pub score: f64,
    pub label: String,
}

#[tracing::instrument(level = "info", ret, err)]
pub async fn predict_toxicity(
    llm_settings: &LLMSettings,
    toxicity_input: ToxicityInput,
) -> Result<bool, SearchError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_bytes(b"Authorization")?,
        HeaderValue::from_str(llm_settings.toxicity_auth_token.expose())?,
    );
    let client = Client::new();

    let response = client
        .post(llm_settings.toxicity_url.as_str())
        .json(&toxicity_input)
        .headers(headers)
        .send()
        .await?;
    let toxicity_api_response: Vec<ToxicityScore> = response.json().await?;

    let toxicity_score = toxicity_api_response
        .into_iter()
        .find(|x| x.label == *"toxic")
        .unwrap_or(ToxicityScore {
            score: 0.0,
            label: String::from(""),
        });
    Ok(toxicity_score.score > llm_settings.toxicity_threshold)
}

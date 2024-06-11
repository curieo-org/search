use serde::{Deserialize, Serialize};
use crate::secrets::Secret;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMSettings {
    pub llm_lingua_url: String,
    pub bio_llm_url: String,
    pub top_k_sources: u16,
    pub toxicity_url: String,
    pub toxicity_threshold: f64,
    pub toxicity_auth_token: Secret<String>,
}

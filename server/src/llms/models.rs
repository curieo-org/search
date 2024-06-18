use crate::secrets::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMSettings {
    pub llm_lingua_url: String,
    pub toxicity_url: String,
    pub toxicity_threshold: f64,
    pub toxicity_auth_token: Secret<String>,
}

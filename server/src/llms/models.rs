use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMSettings {
    pub llm_lingua_url: String,
    pub bio_llm_url: String,
    pub top_k_sources: u16,
}

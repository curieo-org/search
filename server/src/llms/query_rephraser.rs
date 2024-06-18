use crate::llms::LLMSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub query: String,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRephraserInput {
    pub query: String,
    pub previous_context: Vec<QueryResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRephraserOutput {
    pub rephrased_query: String,
}

#[tracing::instrument(level = "debug", ret, err)]
pub async fn rephrase_query(
    llm_settings: &LLMSettings,
    query_rephraser_input: &QueryRephraserInput,
) -> crate::Result<QueryRephraserOutput> {
    Ok(QueryRephraserOutput {
        rephrased_query: query_rephraser_input.query.clone(),
    })
}

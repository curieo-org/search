use color_eyre::eyre::eyre;
// use futures::StreamExt;
use crate::llms::LLMSettings;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMParams {
    pub model: Option<String>,
    pub max_new_tokens: Option<u16>,
    pub temperature: Option<f32>,
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
    pub id: u16,
    pub text: String,
    pub logprob: f32,
    pub special: bool,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BioLLMOutput {
//     pub index: u16,
//     pub token: Token,
//     pub generated_text: Option<String>,
//     pub details: Option<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct BioLLMOutput {
    pub generated_text: String,
}

fn prepare_context_string(open_bio_llm_input: BioLLMInput) -> BioLLMAPIInput {
    BioLLMAPIInput {
        inputs: format!("Context information is below.\n---------------------\n{}\n---------------------\nUse the tools provided, using the most specific tool available for each action.Your final answer should contain all information necessary to answer the question and subquestions.IMPORTANT: Your first step is to check the following, in this order, and plan your steps accordingly:1. Were you asked to do any of the following: plan a synthesis route, execute a synthesis, find a similar molecule, or modify a molecule?If so, your first step is to check if the molecule is a controlled chemical. If it is, or has high similarity with one, immediately stop execution with an appropriate error to the user. Do not continue.2. Does the question involve any molecules? If so, as a first step, check if any are controlled chemicals. If any are, include a warning in your final answer.3. Were you asked to plan a synthesis route? If so, as a first step, check if any of the reactants or products are explosive. If any are, include a warning in your final answer.4. Were you asked to execute a synthesis route? If so, check if any of the reactants or products are explosive. If any are, ask the user for permission to continue.Do not skip these steps.\nQuery: {}\nAnswer: ", open_bio_llm_input.retrieved_result, open_bio_llm_input.query),
            parameters: BioLLMParams {
            model: None,
            max_new_tokens: Some(3000),
            temperature: Some(0.1),
        },
    }
}

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
        .map_err(|e| eyre!("Request to open_bio_llm failed: {e}"))?;

    let open_bio_llm_api_response = response
        .json::<BioLLMOutput>()
        .await
        .map_err(|e| eyre!("Failed to parse open_bio_llm response: {e}"))?;
    Ok(open_bio_llm_api_response)

    // stream the response
    // if response.status().is_success() {
    //     let mut stream = response.bytes_stream();

    //     while let Some(chunk) = stream.next().await {
    //         let chunk = chunk.map_err(|e| eyre!("Failed to read chunk: {e}"))?;
    //         // remove `data` from the start of the chunk and `\n\n` from the end
    //         let chunk = &chunk[5..chunk.len() - 2];
    //         println!("{:?}", chunk);
    //         let open_bio_llm_api_response = serde_json::from_slice::<OpenBioLLMOutput>(&chunk)
    //             .map_err(|e| eyre!("Failed to parse open_bio_llm response: {e}"))?;
    //         println!("{:?}", open_bio_llm_api_response);
    //     }
    // } else {
    //     println!("Request failed with status: {:?}", response.status());
    // }
}

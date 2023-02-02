use serde::{Deserialize, Serialize};

use super::config::ModelConfiguration;
use crate::model_traits::CompletionModel;

const URL: &str = "https://api.openai.com/v1/completions";

#[derive(Debug, Serialize, Deserialize)]
struct CompletionRequest {
    prompt: String,
    #[serde(flatten)]
    model_configuration: ModelConfiguration,
}

impl CompletionRequest {
    fn new(prompt: &str, configuration: ModelConfiguration) -> Self {
        Self {
            prompt: prompt.to_string(),
            model_configuration: configuration,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionLogprobs {
    pub token_logprobs: Vec<Vec<f32>>,
    pub text_offset: Vec<Vec<u32>>,
    pub top_logprobs: Vec<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<CompletionLogprobs>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
}

pub struct CompletionClient {
    api_key: String,
    pub config: ModelConfiguration,
}

impl CompletionClient {
    pub fn new(api_key: String, config: ModelConfiguration) -> Self {
        Self { api_key, config }
    }
}

impl CompletionModel for CompletionClient {
    fn complete(
        &self,
        prompt: &str,
    ) -> Result<std::string::String, std::boxed::Box<(dyn std::error::Error + 'static)>> {
        let request = CompletionRequest::new(prompt, self.config.clone());
        let result = send_completion_request(&self.api_key, &request)?;
        Ok(result.choices[0].text.clone())
    }
}

fn send_completion_request(
    api_key: &str,
    request: &CompletionRequest,
) -> Result<CompletionResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_string() + api_key)
        .header("Content-Type", "application/json")
        .json(request)
        .send()?;

    let result = response.json::<CompletionResponse>()?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::openai::completion::config::ModelConfigurationBuilder;

    #[test]
    fn test_config_builder() {
        let config = ModelConfigurationBuilder::default()
            .model("text-babbage-001".into())
            .max_tokens(64)
            .temperature(0.5)
            .build();

        assert!(config.is_ok());
    }
}

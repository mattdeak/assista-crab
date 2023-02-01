use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json;

const URL: &str = "https://api.openai.com/v1/completions";

#[derive(Debug, Serialize, Deserialize, Builder, Clone, Default)]
#[builder(setter(strip_option), default)]
pub struct ModelConfiguration {
    #[builder(default = "String::from(\"text-babbage-001\")")]
    pub model: String,
    #[builder(default = "64")]
    pub max_tokens: u32,
    #[builder(default = "0.5")]
    pub temperature: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
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

pub struct Client {
    api_key: String,
    pub config: ModelConfiguration,
}

impl Client {
    pub fn new(api_key: String, config: ModelConfiguration) -> Self {
        Self { api_key, config }
    }

    pub fn complete(&self, prompt: &str) -> Result<CompletionResponse, reqwest::Error> {
        let request = CompletionRequest::new(prompt, self.config.clone());
        send_completion_request(&self.api_key, &request)
    }
}

fn send_completion_request(
    api_key: &str,
    request: &CompletionRequest,
) -> Result<CompletionResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let body = serde_json::to_string(&request).unwrap();

    println!("Request: {}", body);

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
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ModelConfigurationBuilder::default()
            .model("text-babbage-001".into())
            .max_tokens(64)
            .temperature(0.5)
            .build();

        if config.is_err() {
            println!("{:?}", config);
        }
        assert!(config.is_ok());
    }
}

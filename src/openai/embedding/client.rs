use crate::model_traits::EmbeddingModel;
use serde::{Deserialize, Serialize};

use super::config::EmbeddingModelConfig;
use super::constants::URL;

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddingRequest {
    input: Vec<String>,

    #[serde(flatten)]
    model_configuration: EmbeddingModelConfig,
}

impl EmbeddingRequest {
    fn new(input: Vec<String>, configuration: EmbeddingModelConfig) -> Self {
        Self {
            input,
            model_configuration: configuration,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
    embedding: Vec<f32>,
    index: u32,

    #[serde(skip_deserializing)]
    object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
    #[serde(skip_deserializing)]
    pub object: String,
}

pub struct EmbeddingClient {
    api_key: String,
    pub config: EmbeddingModelConfig,
}

impl EmbeddingClient {
    pub fn new(api_key: String, config: EmbeddingModelConfig) -> Self {
        Self { api_key, config }
    }
}

impl EmbeddingModel for EmbeddingClient {
    fn embed(&self, documents: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let request = EmbeddingRequest::new(documents.to_vec(), self.config.clone());

        let response = client
            .post(URL)
            .header("Authorization", "Bearer ".to_string() + &self.api_key)
            .json(&request)
            .send()?;

        let result: EmbeddingResponse = response.json()?;

        let embeddings = result.data.into_iter().map(|x| x.embedding).collect();
        Ok(embeddings)
    }
}

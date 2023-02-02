use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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

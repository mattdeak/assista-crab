use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingModelConfig {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EmbeddingModelConfig {
    fn new(model: String) -> Self {
        Self { model, user: None }
    }

    fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl Default for EmbeddingModelConfig {
    fn default() -> Self {
        Self {
            model: String::from("text-embedding-ada-002"),
            user: None,
        }
    }
}

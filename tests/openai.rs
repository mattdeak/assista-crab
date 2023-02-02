extern crate assistant;
use assistant::model_traits::{CompletionModel, EmbeddingModel};
use assistant::openai::completion::client::CompletionClient;
use assistant::openai::completion::config::ModelConfigurationBuilder;

use assistant::openai::embedding::client::EmbeddingClient;
use assistant::openai::embedding::config::EmbeddingModelConfig;

#[test]
fn test_completion() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();

    let model_configuration = ModelConfigurationBuilder::default().build().unwrap();
    let client = CompletionClient::new(api_key, model_configuration);

    let response = client.complete("The meaning of life is the following: ");

    println!("{:?}", response);

    assert!(response.is_ok());
}

#[test]
fn test_completion_with_penalty() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();

    let model_configuration = ModelConfigurationBuilder::default()
        .frequency_penalty(0.4)
        .build()
        .unwrap();

    let client = CompletionClient::new(api_key, model_configuration);

    let response = client.complete("The meaning of life is the following: ");

    println!("{:?}", response);

    assert!(response.is_ok());
}

#[test]
fn test_embedding() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();

    let model_configuration = EmbeddingModelConfig::default();
    let client = EmbeddingClient::new(api_key, model_configuration);

    let response = client.embed_question("The meaning of life is the following: ".to_string());

    println!("{:?}", response);

    assert!(response.is_ok());
}

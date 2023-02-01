#[macro_use]
extern crate derive_builder;

extern crate assistant;
use assistant::openai::completion::Client;
use assistant::openai::completion::ModelConfigurationBuilder;

#[test]
fn test_completion() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();

    let model_configuration = ModelConfigurationBuilder::default().build().unwrap();
    let client = Client::new(api_key, model_configuration);

    let response = client.complete("The meaning of life is the following: ");

    println!("{:?}", response);

    assert!(response.is_ok());
}

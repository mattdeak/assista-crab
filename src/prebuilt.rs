use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::chatbot::Chatbot;
use crate::intent_detector::zeroshot::ZeroShotIntentDetector;
use crate::intent_router::IntentRouter;
use crate::openai::completion::client::CompletionClient;
use crate::openai::completion::config::ModelConfigurationBuilder;
use crate::openai::embedding::client::EmbeddingClient;
use crate::openai::embedding::config::EmbeddingModelConfig;

pub fn build_main_chatbot() -> Chatbot<CompletionClient> {
    let api_key = std::env::var("OPENAI_KEY").unwrap();
    let config = ModelConfigurationBuilder::default()
        .model("text-davinci-003".into())
        .max_tokens(1000)
        .temperature(0.3)
        .top_p(1.0)
        .build()
        .unwrap();

    let client = CompletionClient::new(api_key, config);

    Chatbot::builder(client).prefix("You are a chatbot. Respond to the user, but respond as if you are a pirate. Really embellish, and be very very pirate-like. \n").build()
}

pub fn build_code_execution_chatbot() -> Chatbot<CompletionClient> {
    let api_key = std::env::var("OPENAI_KEY").unwrap();
    let config = ModelConfigurationBuilder::default()
        .model("text-davinci-003".into())
        .max_tokens(256)
        .temperature(0.0)
        .top_p(1.0)
        .build()
        .unwrap();

    let client = CompletionClient::new(api_key, config);

    Chatbot::builder(client)
        .prefix("Write a python script to solve the following problem: ")
        .conversation_limit(0)
        .add_postprocessor(&execute_code)
        .build()
}

fn execute_code(code: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::create("tmpscript.py")?;
    file.write_all(code.as_bytes())?;

    let output = match Command::new("python3").arg("tmpscript.py").output() {
        Ok(_output) => match String::from_utf8(_output.stdout)?.trim() {
            "" => "No output. Could be an issue with the code.".to_string(),
            trimmed_output => trimmed_output.to_string(),
        },
        Err(err) => "Unable to execute code. Result: ".to_string() + &err.to_string(),
    };

    // Remove the script file
    std::fs::remove_file("tmpscript.py")?;

    Ok(output)
}

pub fn build_default_intent_detector() -> ZeroShotIntentDetector<EmbeddingClient> {
    let embeddings_model = EmbeddingClient::new(
        std::env::var("OPENAI_KEY").unwrap(),
        EmbeddingModelConfig::default(),
    );

    let intent_detector = ZeroShotIntentDetector::builder(embeddings_model)
        .with_default_intents()
        .expect("Failed to load default intents")
        .build();

    intent_detector
}

pub fn build_default_router() -> IntentRouter {
    let mut router = IntentRouter::new(Box::new(build_default_intent_detector()));
    router.add_route(
        "code_execution".into(),
        Box::new(build_code_execution_chatbot()),
    );
    router.set_default_route(Box::new(build_main_chatbot()));

    router
}

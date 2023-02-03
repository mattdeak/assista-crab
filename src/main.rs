use std::io::Write;

use assistant::chatbot::Chatbot;
use assistant::intent_detector::intent_detector::IntentDetector;
use assistant::intent_detector::zeroshot::ZeroShotIntentDetector;
use assistant::model_traits::Responder;
use assistant::openai::completion::client::CompletionClient;
use assistant::openai::completion::config::ModelConfigurationBuilder;
use assistant::openai::embedding::client::EmbeddingClient;
use assistant::openai::embedding::config::EmbeddingModelConfig;
use assistant::prebuilt::build_default_router;
use clap::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[command(name = "assistant", author = "Matthew Deakos")]
struct Cli {
    #[clap(short, long, default_value = "false")]
    intent: bool,
}

fn main() {
    let args = Cli::parse();

    match args.intent {
        false => run_router(),
        true => run_intent_detector(),
    }
}

fn run_router() {
    let mut router = build_default_router();
    run_conversation_loop(&mut router);
}

fn run_chatbot() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();
    let config = ModelConfigurationBuilder::default()
        .model("text-davinci-003".into())
        .max_tokens(1000)
        .temperature(0.3)
        .top_p(1.0)
        .build()
        .unwrap();

    let client = CompletionClient::new(api_key, config);
    let mut chatbot = Chatbot::builder(client).prefix("You are a chatbot. Respond to the user, but respond as if you are a pirate. Really embellish, and be very very pirate-like. \n").build();

    run_conversation_loop(&mut chatbot);
}

fn run_conversation_loop<R: Responder>(chatbot: &mut R) {
    loop {
        let mut input = String::new();

        print!("You: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        let response = chatbot.respond(&input).unwrap();

        println!("Assistant: {}", response);
    }
}

fn run_intent_detector() {
    let embeddings_model = EmbeddingClient::new(
        std::env::var("OPENAI_KEY").unwrap(),
        EmbeddingModelConfig::default(),
    );

    let mut intent_detector = ZeroShotIntentDetector::builder(embeddings_model)
        .with_default_intents()
        .expect("Failed to load default intents")
        .build();

    run_intent_detection_loop(&mut intent_detector);
}

fn run_intent_detection_loop<I: IntentDetector>(intent_detector: &mut I) {
    loop {
        let mut input = String::new();

        print!("You: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        let intent = intent_detector.detect_intent(&input).unwrap();

        println!("Intent: {}", intent);
    }
}

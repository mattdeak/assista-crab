use std::io::Write;

use assistant::chatbot::Chatbot;
use assistant::intent_detector::intent_detector::IntentDetector;
use assistant::intent_detector::zeroshot::ZeroShotIntentDetector;
use assistant::model_traits::CompletionModel;
use assistant::openai::completion::client::CompletionClient;
use assistant::openai::completion::config::ModelConfigurationBuilder;
use assistant::openai::embedding::client::EmbeddingClient;
use assistant::openai::embedding::config::EmbeddingModelConfig;
use clap::Parser;

#[derive(Parser)]
#[command(name = "assistant", author = "Matthew Deakos")]
struct Cli {
    #[clap(short, long, default_value = "false")]
    intent: bool,
}

fn main() {
    let args = Cli::parse();

    match args.intent {
        false => run_chatbot(),
        true => run_intent_detector(),
    }
}

fn run_chatbot() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();
    let config = ModelConfigurationBuilder::default()
        .model("text-davinci-003".into())
        .max_tokens(260)
        .temperature(0.5)
        .top_p(1.0)
        .build()
        .unwrap();

    let client = CompletionClient::new(api_key, config);
    let mut chatbot = Chatbot::new(client).build();

    run_conversation_loop(&mut chatbot);
}

fn run_conversation_loop<C: CompletionModel>(chatbot: &mut Chatbot<C>) {
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

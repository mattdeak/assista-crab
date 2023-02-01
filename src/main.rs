mod openai;
use openai::completion::{Client, ModelConfigurationBuilder};

fn main() {
    let api_key = std::env::var("OPENAI_KEY").unwrap();
    let config = ModelConfigurationBuilder::default()
        .max_tokens(60)
        .temperature(0.5)
        .top_p(1.0)
        .build()
        .unwrap();

    let client = Client::new(api_key, config);

    run_conversation_loop(&client);
}

fn run_conversation_loop(client: &Client) {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let response = client.complete(&input).unwrap();

        let choice = response.choices[0].text.clone();

        println!("Assistant: {}", choice);
    }
}

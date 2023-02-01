mod openai;
use openai::completion::{Client, ModelConfigurationBuilder};

fn main() {
    let config = ModelConfigurationBuilder::default()
        .max_tokens(60)
        .temperature(0.5)
        .top_p(1.0)
        .build()
        .unwrap();

    let api_key = std::env::var("OPENAI_KEY").unwrap();

    let client = Client::new(api_key, config);

    let response = client
        .complete("The meaning of life is the following: ")
        .unwrap();

    println!("{:?}", response.choices[0].text);
}

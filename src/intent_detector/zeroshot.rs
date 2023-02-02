use super::intent_detector::{IntentDetector, IntentResult};
use crate::model_traits::EmbeddingModel;
use crate::similarity::cosine_similarity;
use derive_builder::Builder;
use reqwest::get;
use std::error::Error;

#[derive(Clone, PartialEq, Eq)]
pub struct ZeroShotIntent {
    pub intent: String,
    pub training_phrases: Vec<String>,
}

impl ZeroShotIntent {
    fn new(intent: &str, training_phrases: Vec<String>) -> Self {
        Self {
            intent: intent.to_string(),
            training_phrases,
        }
    }
}

pub struct ZeroShotEmbeddedIntent {
    pub intent: String,
    pub embeddings: Vec<Vec<f32>>,
}

pub struct ZeroShotIntentDetectorBuilder<T: EmbeddingModel> {
    embedder: T,
    intents: Vec<ZeroShotIntent>,
}

impl<T: EmbeddingModel> ZeroShotIntentDetectorBuilder<T> {
    pub fn new(embedder: T) -> Self {
        Self {
            embedder,
            intents: Vec::new(),
        }
    }

    pub fn add_intent(mut self, intent: &str, training_phrases: Vec<String>) -> Self {
        self.intents
            .push(ZeroShotIntent::new(intent, training_phrases));
        self
    }

    pub fn add_intents(mut self, intents: Vec<ZeroShotIntent>) -> Self {
        self.intents.extend(intents);
        self
    }

    pub fn with_default_intents(mut self) -> Result<Self, Box<dyn Error>> {
        let intents = get_default_intents();
        self.intents.extend(intents);
        Ok(self)
    }

    pub fn build(self) -> ZeroShotIntentDetector<T> {
        let mut intent_embeddings = Vec::new();

        if self.intents.is_empty() {
            todo!("Either error or add default intents. Not sure which yet.")
        }

        for intent in self.intents {
            let embedding = self
                .embedder
                .embed_answer(&intent.training_phrases)
                .unwrap();

            intent_embeddings.push(ZeroShotEmbeddedIntent {
                intent: intent.intent,
                embeddings: embedding,
            });
        }

        ZeroShotIntentDetector {
            embedder: self.embedder,
            intents: intent_embeddings,
        }
    }
}

pub struct ZeroShotIntentDetector<T: EmbeddingModel> {
    pub embedder: T,
    pub intents: Vec<ZeroShotEmbeddedIntent>,
}

impl<T: EmbeddingModel> ZeroShotIntentDetector<T> {
    pub fn builder(embedder: T) -> ZeroShotIntentDetectorBuilder<T> {
        ZeroShotIntentDetectorBuilder::new(embedder)
    }
}

impl<T: EmbeddingModel> IntentDetector for ZeroShotIntentDetector<T> {
    fn get_intent_scores(&self, text: &str) -> Result<Vec<IntentResult>, Box<dyn Error>> {
        let embedding = self.embedder.embed_question(text.to_string())?;

        let mut scores = Vec::new();
        for intent in &self.intents {
            // get similarity score between the embedding and the intent

            let score = intent
                .embeddings
                .iter()
                .map(|intent_embedding| cosine_similarity(&embedding, intent_embedding))
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            scores.push(IntentResult {
                intent: intent.intent.clone(),
                score,
            });
        }

        Ok(scores)
    }
}

fn get_default_intents() -> Vec<ZeroShotIntent> {
    let intents = vec![
        ZeroShotIntent {
            intent: "greeting".to_string(),
            training_phrases: vec![
                "hello".to_string(),
                "hi".to_string(),
                "how are you".to_string(),
                "how are you doing".to_string(),
                "how are you today".to_string(),
                "hey how's it hanging".to_string(),
                "hey how's it going".to_string(),
                "hey how's it going today".to_string(),
                "hello, nice to meet you".to_string(),
                "hi, nice to meet you".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "goodbye".to_string(),
            training_phrases: vec![
                "goodbye".to_string(),
                "bye".to_string(),
                "see you later".to_string(),
                "see you soon".to_string(),
                "see you".to_string(),
                "talk to you later".to_string(),
                "talk to you soon".to_string(),
                "talk to you".to_string(),
                "have a good day".to_string(),
                "have a good one".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "search_web".to_string(),
            training_phrases: vec![
                "look up how long the wait is at Il Mercato?".to_string(),
                "What's the weather like tomorrow in Paris?".to_string(),
                "Look up the quickest route to Oregon".to_string(),
                "Search the web for trees".to_string(),
                "Google the best restaurants in San Francisco".to_string(),
                "Search the web for the best restaurants in San Francisco".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "search_files".to_string(),
            training_phrases: vec![
                "Search for my resume".to_string(),
                "find any notes from my last meeting".to_string(),
                "search for my notes from my last meeting".to_string(),
                "find any references I have to semantic search".to_string(),
                "search for my references to semantic search".to_string(),
                "Do I have any files related to graph databases".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "casual_chat".to_string(),
            training_phrases: vec![
                "What's your favorite color?".to_string(),
                "What do you think about the philosophy of existentialism?".to_string(),
                "What are some good movies?".to_string(),
                "Are you a robot?".to_string(),
                "I wonder if I'll ever be able to pass the Turing test".to_string(),
                "I've been thinking about the meaning of life lately".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "priming_task".to_string(),
            training_phrases: vec![
                "Help me write a letter".to_string(),
                "Be polite to the customer".to_string(),
                "You are a robot and you are going to help me".to_string(),
                "Summarize the following text".to_string(),
                "Jot down some notes".to_string(),
                "Execute the following code".to_string(),
            ],
        },
        ZeroShotIntent {
            intent: "code_execution".to_string(),
            training_phrases: vec![
                "Tell me how many files are in the current directory".to_string(),
                "What is the total size of the home directory?".to_string(),
                "Give me a word count of the file named test.txt".to_string(),
                "Execute a speed test on the network".to_string(),
                "Show me the top 10 processes by memory usage".to_string(),
                "Show me the top 10 processes by CPU usage".to_string(),
            ],
        },
    ];

    intents
}

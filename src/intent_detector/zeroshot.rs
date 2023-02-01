struct ZeroShotIntent {
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

struct ZeroShotIntentDetector {
    intents: Vec<ZeroShotIntent>,
}

impl ZeroShotIntentDetector {
    fn new(intents: Vec<ZeroShotIntent>) -> Self {
        Self { intents }
    }
}

impl IntentDetector for ZeroShotIntentDetector {
    // Super naive implementation, just counts the number of training phrases
    fn get_intent_scores(&self, text: &str) -> Result<Vec<IntentResult>, Box<dyn Error>> {
        let mut scores = Vec::new();

        for intent in &self.intents {
            let mut score = 0.0;

            for phrase in &intent.training_phrases {
                if text.contains(phrase) {
                    score += 1.0;
                }
            }

            scores.push(IntentResult {
                intent: intent.intent.clone(),
                score,
            });
        }

        Ok(scores)
    }
}

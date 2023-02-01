struct IntentResult {
    intent: String,
    score: f32,
}

pub trait IntentDetector {
    fn get_intent_scores(&self, text: &str) -> Result<Vec<IntentResult>, Box<dyn Error>>;

    fn detect_intent(&self, text: &str) -> Result<IntentResult, Box<dyn Error>> {
        let scores = self.get_intent_scores(text);

        scores
            .into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .unwrap()
    }
}

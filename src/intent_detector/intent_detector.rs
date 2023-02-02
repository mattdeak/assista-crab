use std::error::Error;
use std::fmt::Display;

pub struct IntentResult {
    pub intent: String,
    pub score: f32,
}

impl Display for IntentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Intent: {}, Score: {}", self.intent, self.score)
    }
}

pub trait IntentDetector {
    fn get_intent_scores(&self, text: &str) -> Result<Vec<IntentResult>, Box<dyn Error>>;

    fn detect_intent(&self, text: &str) -> Result<IntentResult, Box<dyn Error>> {
        let scores = self.get_intent_scores(text)?;

        scores
            .into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .ok_or("No intent detected".into())
    }
}

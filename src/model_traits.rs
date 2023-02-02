use std::error::Error;
pub trait CompletionModel {
    fn complete(&self, prompt: &str) -> Result<String, Box<dyn Error>>;
}

pub trait EmbeddingModel {
    fn embed(&self, documents: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn Error>>;

    fn embed_question(&self, text: String) -> Result<Vec<f32>, Box<dyn Error>> {
        match self.embed(&vec![text]) {
            Ok(embeddings) => Ok(embeddings.into_iter().next().unwrap()),
            Err(err) => Err(err),
        }
    }

    fn embed_answer(&self, text: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
        self.embed(text)
    }
}

use std::error::Error;

use crate::model_traits::CompletionModel;

pub struct ChatbotBuilder<T: CompletionModel> {
    model: T,
    conversation_limit: usize,
}

impl<T: CompletionModel> ChatbotBuilder<T> {
    pub fn new(model: T) -> Self {
        Self {
            model,
            conversation_limit: 10,
        }
    }

    pub fn conversation_limit(mut self, limit: usize) -> Self {
        self.conversation_limit = limit;
        self
    }

    pub fn build(self) -> Chatbot<T> {
        Chatbot {
            model: self.model,
            conversation: Vec::new(),
            conversation_limit: self.conversation_limit,
        }
    }
}
pub struct Chatbot<T: CompletionModel> {
    model: T,
    conversation: Vec<String>,
    conversation_limit: usize,
}

impl<T: CompletionModel> Chatbot<T> {
    pub fn new(model: T) -> ChatbotBuilder<T> {
        ChatbotBuilder::new(model)
    }

    fn build_conversation_prompt(&self) -> String {
        self.conversation.join("\n")
    }

    pub fn respond(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let prompt = match self.conversation.len() {
            0 => prompt.to_string(),
            _ => format!("{}\n{}", self.build_conversation_prompt(), prompt),
        };

        let response = self.model.complete(&prompt)?;

        self.conversation.push(prompt);
        self.conversation.push(response.clone());

        if self.conversation.len() > self.conversation_limit {
            self.conversation = self
                .conversation
                .split_off(self.conversation.len() - self.conversation_limit);
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCompletionModel;

    impl CompletionModel for MockCompletionModel {
        fn complete(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
            Ok(prompt.to_string())
        }
    }

    #[test]
    fn test_chatbot() {
        let mut chatbot = Chatbot::new(MockCompletionModel).build();

        let response = chatbot.respond("Hello").unwrap();
        assert_eq!(response, "Hello");

        let response = chatbot.respond("How are you?").unwrap();
        assert_eq!(response, "Hello\nHello\nHow are you?");
    }
}

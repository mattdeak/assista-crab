use std::error::Error;

use crate::model_traits::CompletionModel;

pub struct ChatbotBuilder<T: CompletionModel> {
    model: T,
    conversation_limit: usize,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl<T: CompletionModel> ChatbotBuilder<T> {
    pub fn new(model: T) -> Self {
        Self {
            model,
            conversation_limit: 10,
            prefix: None,
            suffix: None,
        }
    }

    pub fn conversation_limit(mut self, limit: usize) -> Self {
        self.conversation_limit = limit;
        self
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_string());
        self
    }

    pub fn build(self) -> Chatbot<T> {
        Chatbot {
            model: self.model,
            conversation: Vec::new(),
            conversation_limit: self.conversation_limit,
            prefix: self.prefix,
            suffix: self.suffix,
        }
    }
}
pub struct Chatbot<T: CompletionModel> {
    model: T,
    conversation: Vec<String>,
    conversation_limit: usize,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl<T: CompletionModel> Chatbot<T> {
    pub fn builder(model: T) -> ChatbotBuilder<T> {
        ChatbotBuilder::new(model)
    }

    fn build_conversation_prompt(&self) -> String {
        self.conversation.join("\n")
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = Some(prefix.to_string());
    }

    pub fn set_suffix(&mut self, suffix: &str) {
        self.suffix = Some(suffix.to_string());
    }

    pub fn respond(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let prompt = match self.conversation.len() {
            0 => prompt.trim().to_string(),
            _ => format!("{}\n{}", self.build_conversation_prompt(), prompt),
        };

        let prompt = match &self.prefix {
            Some(prefix) => format!("{}{}", prefix, prompt),
            None => prompt,
        };

        let prompt = match &self.suffix {
            Some(suffix) => format!("{}{}", prompt, suffix),
            None => prompt,
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
        let mut chatbot = Chatbot::builder(MockCompletionModel).build();

        let response = chatbot.respond("Hello").unwrap();
        assert_eq!(response, "Hello");

        let response = chatbot.respond("How are you?").unwrap();
        assert_eq!(response, "Hello\nHello\nHow are you?");
    }
}

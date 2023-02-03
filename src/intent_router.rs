use std::collections::HashMap;

use crate::intent_detector::intent_detector::IntentDetector;
use crate::model_traits::Responder;

pub struct IntentRouter {
    detector: Box<dyn IntentDetector>,
    routes: HashMap<String, Box<dyn Responder>>,
    default_route: Option<Box<dyn Responder>>,
}

impl IntentRouter {
    pub fn new(detector: Box<dyn IntentDetector>) -> Self {
        Self {
            detector,
            routes: HashMap::new(),
            default_route: None,
        }
    }

    pub fn add_route(&mut self, intent: String, responder: Box<dyn Responder>) {
        self.routes.insert(intent, responder);
    }

    pub fn set_default_route(&mut self, responder: Box<dyn Responder>) {
        self.default_route = Some(responder);
    }

    pub fn route(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let intent = self.detector.detect_intent(input)?;

        println!("Intent: {}", intent.intent);

        let responder = match self.routes.get_mut(&intent.intent) {
            Some(responder) => responder,
            None => match &mut self.default_route {
                Some(responder) => responder,
                None => return Err("No route found".into()),
            },
        };

        responder.respond(input)
    }
}

impl Responder for IntentRouter {
    fn respond(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.route(input)
    }
}

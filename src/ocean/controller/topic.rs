use super::Controller;
use serde::Deserialize;
use serde_json;

pub struct Topic {}

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
    description: String,
}

impl Topic {
    fn create(&self, params: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let request: CreateRequest = serde_json::from_value(params.unwrap()).unwrap();
        None
    }
}

impl Controller for Topic {
    fn new() -> Topic {
        Topic {}
    }

    fn exec(&self, method: &str, params: Option<serde_json::Value>) -> Option<serde_json::Value> {
        match method {
            "create" => self.create(params),
            _ => {
                println!("method {} not found", method);
                None
            }
        }
    }
}

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
    fn create(&self, params: serde_json::Value) {
        let request: CreateRequest = serde_json::from_value(params).unwrap();
        println!("name: {} descr: {}", request.name, request.description);
    }
}

impl Controller for Topic {
    fn new() -> Topic {
        Topic {}
    }

    fn exec(&self, method: &str, params: serde_json::Value) {
        println!("exec {}", method);

        match method {
            "create" => self.create(params),
            _ => println!("method {} not found", method),
        }
    }
}

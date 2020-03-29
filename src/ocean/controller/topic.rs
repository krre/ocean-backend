use super::Controller;
use serde_json;

#[derive(Default)]
pub struct Topic {
    name: String,
    description: String,
}

impl Topic {}

impl Controller for Topic {
    fn new() -> Topic {
        Default::default()
    }

    fn exec(&self, method: &str, params: &Option<serde_json::Value>) {
        println!("exec {}", method);

        match method {
            "create" => println!("run create"),
            _ => println!("method {} not found", method),
        }
    }
}

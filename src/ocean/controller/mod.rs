use serde_json;

pub mod topic;

pub trait Controller {
    fn new() -> Self;
    fn exec(&self, method: &str, params: serde_json::Value);
}

use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub method: String,
    pub params: serde_json::Value,
}

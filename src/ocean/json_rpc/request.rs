use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

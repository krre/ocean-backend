use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub id: Option<String>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

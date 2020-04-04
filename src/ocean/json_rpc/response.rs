use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub method: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<Error>,
}

#[derive(Serialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

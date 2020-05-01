use crate::api::error;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Serialize)]
pub struct Error {
    pub code: error::ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl Error {
    pub fn new(code: error::ErrorCode, data: Option<String>) -> Error {
        Error {
            code: code,
            message: error::message(code),
            data: data,
        }
    }
}

use crate::api;
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
    pub code: api::error::ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl Error {
    pub fn new(code: api::error::ErrorCode, message: String, data: Option<String>) -> Error {
        Error {
            code,
            message,
            data,
        }
    }

    pub fn from_api_error(err: api::error::Error) -> Error {
        Self::new(err.code(), err.message(), err.data())
    }
}

use crate::api;
use serde::Serialize;

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

    pub fn from_api_error(err: &api::error::Error) -> Error {
        Self::new(err.code(), err.message(), err.data())
    }
}

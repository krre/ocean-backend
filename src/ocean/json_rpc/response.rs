use serde::Serialize;
use serde_json;
use std::collections::HashMap;

pub type ErrorCode = i32;

// Common (1..99)
pub const PARSE_ERROR: ErrorCode = 1;
pub const CONTROLLER_NOT_FOUND: ErrorCode = 2;
pub const METHOD_NOT_FOUND: ErrorCode = 3;
pub const PARAMETER_NOT_FOUND: ErrorCode = 4;

// User (100..199)
pub const INVALID_PAIR_ID_PASSWORD_: ErrorCode = 100;

lazy_static! {
    static ref ERROR_MESSAGES: HashMap<ErrorCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(PARSE_ERROR, "Parse error");
        m.insert(CONTROLLER_NOT_FOUND, "Controller not found");
        m.insert(METHOD_NOT_FOUND, "Method not found");
        m.insert(PARAMETER_NOT_FOUND, "Parameter not found");
        m.insert(INVALID_PAIR_ID_PASSWORD_, "Invalid pair `id / password`");
        m
    };
}

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
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl Error {
    pub fn new(code: ErrorCode, data: Option<String>) -> Error {
        Error {
            code: code,
            message: ERROR_MESSAGES.get(&code).unwrap().to_string(),
            data: data,
        }
    }
}

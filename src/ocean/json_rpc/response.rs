use serde_json;

pub struct Response {
    id: String,
    method: String,
    result: Option<serde_json::Value>,
    error: Option<Error>,
}

pub struct Error {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

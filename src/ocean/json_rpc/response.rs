use crate::json_rpc::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            id: "".to_string(),
            method: "".to_string(),
            result: None,
            error: None,
        }
    }
}

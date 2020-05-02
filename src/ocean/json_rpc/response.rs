use crate::json_rpc::Error;
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

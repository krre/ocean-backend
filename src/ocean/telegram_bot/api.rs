use crate::types::Id;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: Id,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub chat: Chat,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: Id,
}

#[derive(Serialize, Debug)]
pub struct GetUpdatesParams {
    pub offset: i32,
}

#[derive(Serialize, Debug)]
pub struct SendMessageParams {
    pub chat_id: String,
    pub text: String,
    pub parse_mode: Option<String>,
}

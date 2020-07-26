use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub chat: Chat,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i32,
}

#[derive(Serialize, Debug)]
pub struct SendMessageParams {
    pub chat_id: i32,
    pub text: String,
}

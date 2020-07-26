use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    pub chat: Chat,
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i32,
}

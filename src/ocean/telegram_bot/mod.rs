use crate::config;
use crate::db;
use chrono;
use diesel::prelude::*;
use log::error;
use reqwest;
use timer;

pub mod api;

pub struct TelegramBot {
    _guard: timer::Guard,
    _timer: timer::Timer,
}

impl TelegramBot {
    pub fn new() -> Self {
        let timer = timer::Timer::new();
        let guard = timer.schedule_repeating(
            chrono::Duration::seconds(config::CONFIG.telegram_bot.interval),
            move || {
                get_new_users();
            },
        );

        Self {
            _guard: guard,
            _timer: timer,
        }
    }
}

fn get_new_users() {
    let db = db::Db::new();
    let mut offset = get_offset(&db) + 1;
    let params = api::GetUpdatesParams { offset };
    let res = send_request("getUpdates", serde_json::to_value(params).unwrap());

    if res == serde_json::Value::Null {
        return;
    }

    let updates: Vec<api::Update> = serde_json::from_value(res).unwrap();

    for update in &updates {
        offset = update.update_id;
        let text = &update.message.text;
        let chat_id = update.message.chat.id;

        if text == "/start" {
            if insert_chat_id(chat_id, &db) {
                send_message(
                    chat_id,
                    "Вы успешно подписаны на рассылку уведомлений".into(),
                );
            }
        } else if text == "/stop" {
            if delete_chat_id(chat_id, &db) {
                send_message(
                    chat_id,
                    "Вы успешно отписаны от рассылки уведомлений".into(),
                );
            }
        }
    }

    if updates.len() > 0 {
        update_offset(offset, &db);
    }
}

fn get_offset(db: &db::Db) -> i32 {
    use crate::model::schema::values::dsl::*;
    let res = values
        .select(value)
        .filter(name.eq("telegram_update_id"))
        .first::<Option<serde_json::Value>>(&db.conn)
        .unwrap();

    let offset = res.unwrap();
    serde_json::from_value(offset).unwrap()
}

fn update_offset(offset: i32, db: &db::Db) {
    use crate::model::schema::values::dsl::*;
    use serde_json::json;
    diesel::update(values.filter(name.eq("telegram_update_id")))
        .set(value.eq(json!(offset)))
        .execute(&db.conn)
        .unwrap();
}

fn insert_chat_id(user_chat_id: i32, db: &db::Db) -> bool {
    use crate::model::schema::telegram_chats::dsl::*;

    if find_chat_id(user_chat_id, &db) {
        return false;
    }

    diesel::insert_into(telegram_chats)
        .values(chat_id.eq(user_chat_id))
        .execute(&db.conn)
        .unwrap();
    true
}

fn delete_chat_id(user_chat_id: i32, db: &db::Db) -> bool {
    use crate::model::schema::telegram_chats::dsl::*;

    if !find_chat_id(user_chat_id, &db) {
        return false;
    }

    diesel::delete(telegram_chats.filter(chat_id.eq(user_chat_id)))
        .execute(&db.conn)
        .unwrap();

    true
}

fn find_chat_id(user_chat_id: i32, db: &db::Db) -> bool {
    use crate::model::schema::telegram_chats::dsl::*;

    let res = telegram_chats
        .select(id)
        .filter(chat_id.eq(user_chat_id))
        .first::<i32>(&db.conn)
        .optional()
        .unwrap();

    if let Some(_r) = res {
        return true;
    } else {
        return false;
    }
}

pub fn send_message(chat_id: i32, text: String) {
    let params = api::SendMessageParams { chat_id, text };
    send_request("sendMessage", serde_json::to_value(params).unwrap());
}

pub fn send_message_to_all(text: &String, db: &db::Db) {
    use crate::model::schema::telegram_chats::dsl::*;

    let chat_ids = telegram_chats
        .select(chat_id)
        .load::<i32>(&db.conn)
        .unwrap();

    for user_chat_id in chat_ids {
        send_message(user_chat_id, text.clone());
    }
}

#[tokio::main]
async fn send_request(method: &str, params: serde_json::Value) -> serde_json::Value {
    let url = make_url(method);
    let res = send(url, params).await;

    match res {
        Ok(r) => return r,
        Err(e) => {
            error!("Telegram API request error: {:?}", e);
            return serde_json::Value::Null;
        }
    }
}

async fn send(
    url: String,
    params: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&url)
        .json(&params)
        .send()
        .await?
        .json::<api::Response>()
        .await?;

    if !resp.ok {
        error!("Telegram API response error: {}", resp.description.unwrap());
        return Ok(serde_json::Value::Null);
    }

    Ok(resp.result.unwrap())
}

fn make_url(method: &str) -> String {
    config::CONFIG.telegram_bot.url.clone()
        + "/bot"
        + &config::CONFIG.telegram_bot.token
        + "/"
        + method
}

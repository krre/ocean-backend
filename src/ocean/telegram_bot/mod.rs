use crate::config;
use chrono;
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
    let res = send_request("getUpdates", serde_json::Value::Null);

    if res == serde_json::Value::Null {
        return;
    }

    let updates: Vec<api::Update> = serde_json::from_value(res).unwrap();

    for update in updates {
        let update_id = update.update_id;
        let text = update.message.text;

        if text != "/start" {
            continue;
        }

        let chat_id = update.message.chat.id;
        // send_message(chat_id, "HELLO".into());
        println!("{} {} {}", update_id, text, chat_id);
    }
}

pub fn send_message(chat_id: i32, text: String) {
    let params = api::SendMessageParams { chat_id, text };
    send_request("sendMessage", serde_json::to_value(params).unwrap());
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

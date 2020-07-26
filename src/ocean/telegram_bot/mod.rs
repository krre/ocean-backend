use crate::config;
use chrono;
use log::{error, info};
use reqwest;
use std::collections::HashMap;
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
    let res = send_request("getUpdates");

    if res == serde_json::Value::Null {
        return;
    }

    println!("{:?}", res);
}

#[tokio::main]
async fn send_request(method: &str) -> serde_json::Value {
    let url = make_url(method);
    let res = send(url).await;

    match res {
        Ok(r) => return r,
        Err(e) => {
            error!("Telegram API request error: {:?}", e);
            return serde_json::Value::Null;
        }
    }
}

async fn send(url: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url).await?.json::<api::Response>().await?;

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

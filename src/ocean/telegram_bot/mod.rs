use crate::config;
use chrono;
use reqwest;
use std::collections::HashMap;
use timer;

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
    println!("tick!");
    let res = make_request();
}

#[tokio::main]
async fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let url = make_url("getUpdates");
    println!("{}", url);

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn make_url(method: &str) -> String {
    config::CONFIG.telegram_bot.url.clone()
        + "/bot"
        + &config::CONFIG.telegram_bot.token
        + "/"
        + method
}

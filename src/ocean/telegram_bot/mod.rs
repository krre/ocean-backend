use crate::config;
use log::error;
use reqwest;

pub mod api;

pub fn send_message(text: String) {
    let params = api::SendMessageParams {
        chat_id: config::CONFIG.telegram_bot.channel.clone(),
        text,
        parse_mode: Some("HTML".into()),
    };

    use std::thread;
    thread::spawn(move || {
        send_request("sendMessage", serde_json::to_value(params).unwrap());
    });
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

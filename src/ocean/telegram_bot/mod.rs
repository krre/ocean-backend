use crate::config;
use log::error;
use reqwest;

pub mod api;

pub fn send_message(text: String) {
    send_message_to(config::CONFIG.telegram_bot.channel.clone(), text);
}

pub fn send_admin_message(text: String) {
    send_message_to(config::CONFIG.telegram_bot.admin_chat_id.clone(), text);
}

fn send_message_to(chat_id: String, text: String) {
    let mut adaptive_text = text;

    const TEXT_LIMIT: usize = 4096; // Telegram Bot limit

    if adaptive_text.len() > TEXT_LIMIT {
        const CUT_SIGN: &str = "[...]";
        const SAFE_CUT_LENGHT: usize = 16; // Safe cutting numbers of symbols to fit to UTF char boundary
        adaptive_text.truncate(TEXT_LIMIT - SAFE_CUT_LENGHT);
        adaptive_text.push_str(CUT_SIGN);
    }

    let params = api::SendMessageParams {
        chat_id: chat_id,
        text: adaptive_text,
        parse_mode: Some("HTML".into()),
    };

    std::thread::spawn(move || {
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

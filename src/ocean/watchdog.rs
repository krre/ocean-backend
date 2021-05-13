use crate::config;
use crate::json_rpc;
use log::{error, info};
use std::thread;
use std::time;
use url::Url;

pub fn start() {
    thread::spawn(|| loop {
        thread::sleep(time::Duration::from_secs(60));
        info!("Heartbeat");

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let mut url = Url::parse("https://localhost/api").unwrap();
        url.set_port(Some(config::CONFIG.server.port)).unwrap();
        url.set_query(Some(&format!(
            "token={}",
            config::CONFIG.watchdog.anonym_token
        )));

        let req = json_rpc::Request {
            method: "ping".to_string(),
            id: Some("1".to_string()),
            params: None,
        };

        let json = serde_json::to_value(&req).unwrap();
        let resp = client.post(url).json(&json).send();

        if let Err(e) = resp {
            error!("Watchdog request error: {:?}", e);
            std::process::exit(0);
        }
    });

    info!("Watchdog started");
}

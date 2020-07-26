use crate::api::server;
use crate::config;
use crate::telegram_bot::TelegramBot;

pub struct App {
    _telegram_bot: Option<TelegramBot>,
}

impl App {
    pub fn new() -> Self {
        let bot = if config::CONFIG.telegram_bot.enabled {
            Some(TelegramBot::new())
        } else {
            None
        };

        App { _telegram_bot: bot }
    }

    pub async fn start(&self) {
        let server = server::Server::new();
        server.listen().await;
    }
}

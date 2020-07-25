use crate::api::server;
use crate::telegram_bot::TelegramBot;

pub struct App {
    _telegram_bot: TelegramBot,
}

impl App {
    pub fn new() -> Self {
        let bot = TelegramBot::new();

        App { _telegram_bot: bot }
    }

    pub async fn start(&self) {
        let server = server::Server::new();
        server.listen().await;
    }
}

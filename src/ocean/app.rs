use crate::api::server;
use crate::config;
use crate::trash_monitor;
use crate::watchdog;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if config::CONFIG.watchdog.enabled {
            watchdog::start();
        }

        trash_monitor::start();

        let server = server::ApiServer::new();
        server.listen().await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

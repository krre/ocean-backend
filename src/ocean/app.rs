use crate::api::server;
use crate::watchdog;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        watchdog::start();

        let server = server::ApiServer::new();
        server.listen().await?;
        Ok(())
    }
}

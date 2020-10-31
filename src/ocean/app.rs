use crate::api::server;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = server::ApiServer::new();
        server.listen().await?;
        Ok(())
    }
}

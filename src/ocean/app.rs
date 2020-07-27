use crate::api::server;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) {
        let server = server::Server::new();
        server.listen().await;
    }
}

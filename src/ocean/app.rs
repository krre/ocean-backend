use crate::api::server;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn start(&self) {
        let server = server::Server::new();
        server.listen().await;
    }
}

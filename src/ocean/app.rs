use crate::api::server;

pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    pub async fn start(&self) {
        let server = server::Server::new();
        server.listen().await;
    }
}

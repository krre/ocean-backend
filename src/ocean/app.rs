use crate::api_server;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub async fn start(&self) {
        let server = api_server::ApiServer::new();
        server.listen().await;
    }
}

use crate::api_server;
use crate::db;
use crate::migration;
use tokio::task;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub async fn start(&self) {
        task::spawn_blocking(|| {
            let mut db = db::Db::new();
            migration::migrate(&mut db);
        })
        .await
        .unwrap();

        let server = api_server::ApiServer::new();
        server.listen().await;
    }
}

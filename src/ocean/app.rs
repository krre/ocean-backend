use crate::api_server;
use crate::config;
use crate::db;
use crate::migration;
use tokio::task;

pub struct App {
    config: config::Config,
}

impl App {
    pub fn new(config: config::Config) -> App {
        App { config }
    }

    pub async fn start(&self) {
        let cfg = self.config.postgres.clone();

        task::spawn_blocking(|| {
            let mut db = db::Db::new(cfg);
            migration::migrate(&mut db);
        })
        .await
        .unwrap();

        let server = api_server::ApiServer::new(self.config.server.port);
        server.listen().await;
    }
}

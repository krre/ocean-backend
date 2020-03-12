use crate::api_server;
use crate::config;
use crate::db;
use crate::migration;

pub struct App {
    config: config::Config,
}

impl App {
    pub fn new(config: config::Config) -> App {
        App { config }
    }

    pub fn start(&self) {
        let mut db = db::Db::new(&self.config.postgres);
        migration::migrate(&mut db);

        let server = api_server::ApiServer::new(self.config.server.port);
        server.listen();
    }
}

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
        let db = db::Db::new(&self.config.postgres);
        migration::migrate(&db);
    }
}

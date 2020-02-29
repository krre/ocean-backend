use crate::config;
use crate::db;

pub struct App {}

impl App {
    pub fn new(config: &config::Config) -> App {
        let db = db::Db::new(&config.postgres);
        App {}
    }

    pub fn start(&self) {
        println!("Ocean started");
    }
}

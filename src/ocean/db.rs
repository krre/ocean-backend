use crate::config;
use postgres::{Client, NoTls};

pub struct Db {
    pub conn: Client,
}

impl Db {
    pub fn new(config: &config::Postgres) -> Db {
        let conn = Client::configure()
            .host("localhost")
            .port(config.port)
            .dbname(&config.database)
            .user(&config.username)
            .password(&config.password)
            .connect(NoTls)
            .unwrap();
        Db { conn }
    }
}

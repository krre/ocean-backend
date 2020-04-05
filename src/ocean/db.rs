use crate::config;
use postgres::{Client, NoTls};

pub struct Db {
    pub conn: Client,
}

impl Db {
    pub fn new() -> Db {
        let conn = Client::configure()
            .host("localhost")
            .port(config::CONFIG.postgres.port)
            .dbname(&config::CONFIG.postgres.database)
            .user(&config::CONFIG.postgres.username)
            .password(&config::CONFIG.postgres.password)
            .connect(NoTls)
            .unwrap();
        Db { conn }
    }
}

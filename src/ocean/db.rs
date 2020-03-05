use crate::config;
use postgres::{Client, NoTls};

pub struct Db {
    client: postgres::Client,
}

impl Db {
    pub fn new(config: &config::Postgres) -> Db {
        let client = Client::configure()
            .host("localhost")
            .port(config.port)
            .dbname(&config.database)
            .user(&config.username)
            .password(&config.password)
            .connect(NoTls)
            .unwrap();
        Db { client }
    }

    pub fn migrate(&self) {
        print!("Database migration... ");
        println!("OK")
    }
}

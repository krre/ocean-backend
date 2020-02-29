use crate::config;

pub struct Db {}

impl Db {
    pub fn new(config: &config::Postgres) -> Db {
        Db {}
    }
}

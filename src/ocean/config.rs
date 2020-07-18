use serde_derive::Deserialize;
use std::fs;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub postgres: Postgres,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub ssl: Ssl,
}

#[derive(Debug, Deserialize)]
pub struct Ssl {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("ocean/ocean.toml");

        if !config_path.exists() {
            panic!("config path not exists: {}", config_path.to_str().unwrap());
        }

        let config_text = fs::read_to_string(config_path).unwrap();
        toml::from_str(&config_text).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

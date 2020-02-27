use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: Postgres,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Config {
    pub fn new() -> Config {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("ocean/ocean.toml");

        if !config_path.exists() {
            panic!("config path not exists: {}", config_path.to_str().unwrap());
        }

        let config_text = fs::read_to_string(config_path).unwrap();
        let config: Config = toml::from_str(&config_text).unwrap();
        config
    }
}

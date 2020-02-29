use crate::config;

pub struct App {}

impl App {
    pub fn new(config: &config::Config) -> App {
        App {}
    }

    pub fn start(&self) {
        println!("Ocean started");
    }
}

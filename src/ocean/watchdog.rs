use log::info;
use std::thread;
use std::time;

pub fn start() {
    thread::spawn(|| loop {
        thread::sleep(time::Duration::from_secs(60));
        info!("Heartbeat");
    });
    info!("Watchdog started");
}

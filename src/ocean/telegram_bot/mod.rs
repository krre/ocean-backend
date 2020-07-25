use crate::config;
use chrono;
use timer;

pub struct TelegramBot {
    _guard: timer::Guard,
    _timer: timer::Timer,
}

impl TelegramBot {
    pub fn new() -> Self {
        let timer = timer::Timer::new();
        let guard = timer.schedule_repeating(
            chrono::Duration::seconds(config::CONFIG.telegram_bot.interval),
            move || {
                get_new_users();
            },
        );

        Self {
            _guard: guard,
            _timer: timer,
        }
    }
}

fn get_new_users() {
    println!("tick!");
}

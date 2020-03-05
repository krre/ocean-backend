use ocean::app;
use ocean::config;

fn main() {
    println!("Ocean started");

    let config = config::Config::new();
    let app = app::App::new(config);
    app.start();
}

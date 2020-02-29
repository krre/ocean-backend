use ocean::app;
use ocean::config;

fn main() {
    let config = config::Config::new();
    println!("{:?}", config);

    let app = app::App::new(&config);
    app.start();
}

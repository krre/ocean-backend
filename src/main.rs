use ocean::app;
use ocean::config;

#[tokio::main]
async fn main() {
    println!("Ocean started");

    let config = config::Config::new();
    let app = app::App::new(config);
    app.start().await;
}

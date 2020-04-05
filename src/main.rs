use ocean::app;

#[tokio::main]
async fn main() {
    println!("Ocean started");

    let app = app::App::new();
    app.start().await;
}

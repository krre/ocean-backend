#[macro_use]
extern crate diesel_migrations;
use ocean::app;
use ocean::db;

embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    println!("Ocean started");

    let db = db::Db::new();
    embedded_migrations::run_with_output(&db.conn, &mut std::io::stdout()).unwrap();

    let app = app::App::new();
    app.start().await;
}

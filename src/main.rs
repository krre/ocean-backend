#[macro_use]
extern crate diesel_migrations;
use log::info;
use ocean::api::user_cache;
use ocean::app;
use ocean::db;

embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::builder().format_timestamp(None).init();

    info!("Ocean started");

    let db = db::Db::new();
    embedded_migrations::run_with_output(&db.conn, &mut std::io::stdout()).unwrap();

    user_cache::init(db);

    let app = app::App::new();
    app.start().await?;
    Ok(())
}

extern crate diesel_migrations;
use diesel_migrations::MigrationHarness;
use log::info;
use ocean::api::user_cache;
use ocean::app;
use ocean::db;

use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::builder().format_timestamp(None).init();

    info!("Ocean started");

    let mut db = db::Db::new();
    db.conn.run_pending_migrations(MIGRATIONS)?;

    user_cache::init(db);

    let app = app::App::new();
    app.start().await?;
    Ok(())
}

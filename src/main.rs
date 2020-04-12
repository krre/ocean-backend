#[macro_use]
extern crate diesel_migrations;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use ocean::app;
use ocean::config;

embed_migrations!("migrations");

pub fn establish_connection() -> PgConnection {
    let database_url = format!(
        "postgres://{}:{}@localhost/{}",
        config::CONFIG.postgres.username,
        config::CONFIG.postgres.password,
        config::CONFIG.postgres.database
    );
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    println!("Ocean started");

    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let app = app::App::new();
    app.start().await;
}

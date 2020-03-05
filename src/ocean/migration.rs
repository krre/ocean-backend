use crate::db;

const VERSION: u16 = 0;

pub fn migrate(db: &db::Db) {
    print!("Database migration... ");
    println!("OK")
}

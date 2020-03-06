use crate::db;

const VERSION: u16 = 0;

pub fn migrate(db: &mut db::Db) {
    println!("Start database migration");
    if db.has_table("migrations") {
        println!("run patches");
    } else {
        println!("create migrations");
    }
    println!("Finish database migration")
}

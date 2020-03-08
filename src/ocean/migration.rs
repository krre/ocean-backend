use crate::db;

const VERSION: usize = 1;
const PATCHES: [fn(db: &mut db::Db); VERSION] = [patch1];

pub fn migrate(db: &mut db::Db) {
    println!("Start database migration");

    let mut last_version = 0;

    if db.has_table("migrations") {
        let row = db
            .conn
            .query_one("SELECT id FROM migrations ORDER BY id DESC LIMIT 1", &[])
            .unwrap();

        let id: i32 = row.get("id");
        last_version = id as usize;
    }

    if last_version == VERSION {
        println!("Nothing to patch");
        return;
    }

    let mut i = last_version;

    for patch in &PATCHES[last_version..VERSION] {
        i += 1;
        println!("Apply database patch {}", i);
        patch(db);
        // Create row with id as version number started from 1
        db.conn
            .execute("INSERT INTO migrations (created_at) VALUES (now())", &[])
            .unwrap();
    }

    println!("Finish database migration")
}

fn exec_queries(db: &mut db::Db, queries: &[&str]) {
    for query in queries {
        db.conn.execute(*query, &[]).unwrap();
    }
}

fn patch1(db: &mut db::Db) {
    let queries = ["CREATE TABLE IF NOT EXISTS migrations (
        id serial NOT NULL PRIMARY KEY,
        created_at timestamptz NOT NULL DEFAULT now());"];

    exec_queries(db, &queries);
}

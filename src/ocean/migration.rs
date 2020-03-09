use crate::db;

const VERSION: usize = 1;
const PATCHES: [fn(db: &mut db::Db); VERSION] = [patch1];

pub fn migrate(db: &mut db::Db) {
    let last_version = last_version(db) as usize;

    println!("Database version: {}", last_version);

    if last_version == VERSION {
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
}

fn last_version(db: &mut db::Db) -> i32 {
    if VERSION == 1 {
        db.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS migrations (
                id serial NOT NULL PRIMARY KEY,
                created_at timestamptz NOT NULL DEFAULT now())",
                &[],
            )
            .unwrap();
    }

    let row = db
        .conn
        .query_one("SELECT id FROM migrations ORDER BY id DESC LIMIT 1", &[]);

    match row {
        Ok(row) => row.get("id"),
        Err(_) => 0,
    }
}

fn exec_queries(db: &mut db::Db, queries: &[&str]) {
    for query in queries {
        db.conn.execute(*query, &[]).unwrap();
    }
}

fn patch1(db: &mut db::Db) {
    let queries = ["
        CREATE TABLE IF NOT EXISTS topics (
        id serial NOT NULL PRIMARY KEY,
        title text NOT NULL,
        description text NOT NULL,
        updated_at timestamptz NOT NULL DEFAULT now(),
        created_at timestamptz NOT NULL DEFAULT now())"];

    exec_queries(db, &queries);
}

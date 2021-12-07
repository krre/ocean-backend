use crate::db;
use diesel::prelude::*;
use log::info;
use std::thread;
use std::time;

pub fn start() {
    thread::spawn(|| {
        let db = db::Db::new();
        process_mandels(&db);

        loop {
            thread::sleep(time::Duration::from_secs(12 * 60 * 60)); // 12 hours
            process_mandels(&db);
        }
    });

    info!("Trash monitor started");
}

fn process_mandels(db: &db::Db) {
    use diesel::sql_types::Int4;

    // Move to trash (for mandels older then 2 days)
    let moved = diesel::sql_query(
        "UPDATE mandels AS m SET trash = true
        WHERE trash = false AND (SELECT (EXTRACT(epoch FROM (SELECT (now() - create_ts))) / 86400)::int) >= 2
            AND ((SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $1) > (SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $2)
            OR (SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $2) < 4)"
        )
        .bind::<Int4, _>(crate::types::Vote::Fake as i32)
        .bind::<Int4, _>(crate::types::Vote::Yes as i32)
        .execute(&db.conn).expect("Failed to move mandels to trash");

    info!("Moved to trash {} mandels", moved);

    // Restore from trash
    let restored = diesel::sql_query(
        "UPDATE mandels AS m SET trash = false
        WHERE trash = true AND
            ((SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $1) <= (SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $2)
            AND (SELECT count(*) FROM votes WHERE mandela_id = m.id AND vote = $2) >= 4)"
        )
        .bind::<Int4, _>(crate::types::Vote::Fake as i32)
        .bind::<Int4, _>(crate::types::Vote::Yes as i32)
        .execute(&db.conn).expect("Failed to restore mandels from trash");

    info!("Restored from trash {} mandels", restored);
}

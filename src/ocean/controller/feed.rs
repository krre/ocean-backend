use super::*;
use crate::types::Id;
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::sql_types::Int8;
use diesel::sql_types::Text;
use diesel::sql_types::Timestamptz;
use serde::Deserialize;
use serde::Serialize;

// feed.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        limit: i32,
        offset: i32,
    }

    let req: Req = data.params()?;

    #[derive(QueryableByName, Serialize)]
    struct Feed {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int8"]
        row: i64,
        #[sql_type = "Int4"]
        title_id: Id,
        #[sql_type = "Text"]
        title: String,
        #[sql_type = "Text"]
        message: String,
        #[sql_type = "Int4"]
        user_id: Id,
        #[sql_type = "Text"]
        user_name: String,
        #[sql_type = "Text"]
        #[serde(rename(serialize = "type"))]
        type_: String,
        #[sql_type = "Timestamptz"]
        create_ts: NaiveDateTime,
    }

    let feeds = sql_query(
        "SELECT c.id, rank() OVER (PARTITION BY mandela_id ORDER BY c.id ASC) AS row, m.id AS title_id,
            (CASE WHEN m.title_mode = 0 THEN m.title ELSE m.what || ': ' || m.before || ' / ' || m.after END) AS title,
            message, c.user_id, u.name AS user_name, c.create_ts, 'comment' AS type_
        FROM comments AS c
            JOIN mandels AS m ON m.id = c.mandela_id
            JOIN users AS u ON u.id = c.user_id
        UNION
        SELECT fp.id, rank() OVER (PARTITION BY topic_id ORDER BY fp.id ASC) AS row,
            ft.id AS title_id, ft.name AS title, post AS message, fp.user_id, u.name AS user_name, fp.create_ts, 'post' AS type_
        FROM forum_posts AS fp
            JOIN forum_topics AS ft ON ft.id = fp.topic_id
            JOIN users AS u ON u.id = fp.user_id
        UNION
        SELECT 0 AS id, 0 AS row, m.id AS title_id, (CASE WHEN m.title_mode = 0 THEN m.title ELSE m.what || ': ' || m.before || ' / ' || m.after END) AS title,
            description AS message, user_id, u.name AS user_name, m.create_ts, 'mandela' AS type_
        FROM mandels AS m
            JOIN users AS u on u.id = m.user_id
        UNION
        SELECT 0 AS id, 0 AS row, ft.id AS title_id, ft.name AS title, '' AS message, user_id, u.name AS user_name, ft.create_ts, 'topic' AS type_
        FROM forum_topics AS ft
            JOIN users AS u ON u.id = ft.user_id
        ORDER BY create_ts DESC
        LIMIT $1
        OFFSET $2",
    )
    .bind::<Int4, _>(req.limit)
    .bind::<Int4, _>(req.offset)
    .load::<Feed>(&data.db.conn)?;

    #[derive(QueryableByName)]
    struct TotalCount {
        #[sql_type = "Int8"]
        mandels_count: i64,
        #[sql_type = "Int8"]
        comments_count: i64,
        #[sql_type = "Int8"]
        forum_topics_count: i64,
        #[sql_type = "Int8"]
        forum_posts_count: i64,
    }

    let total_counts = sql_query(
        "SELECT
            (SELECT COUNT(*) FROM mandels) AS mandels_count,
            (SELECT COUNT(*) FROM comments) AS comments_count,
            (SELECT COUNT(*) FROM forum_topics) AS forum_topics_count,
            (SELECT COUNT(*) FROM forum_posts) AS forum_posts_count",
    )
    .load::<TotalCount>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        feeds: Vec<Feed>,
        total_count: i64,
    }

    let total_count = if total_counts.len() > 0 {
        total_counts[0].mandels_count
            + total_counts[0].comments_count
            + total_counts[0].forum_topics_count
            + total_counts[0].forum_posts_count
    } else {
        0
    };

    let resp = Resp {
        total_count: total_count,
        feeds: feeds,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

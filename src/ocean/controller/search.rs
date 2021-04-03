use super::*;
use crate::types::Id;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::{Int4, Int8, Text};
use serde::{Deserialize, Serialize};

// search.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        text: String,
        #[serde(rename(deserialize = "type"))]
        type_: i8,
        offset: i64,
        limit: i64,
    }

    let req: Req = data.params()?;

    #[derive(Serialize)]
    struct Resp {
        records: Vec<Record>,
        total_count: i64,
    }

    if req.text.is_empty() {
        let resp = Resp {
            records: Vec::new(),
            total_count: 0,
        };
        let result = serde_json::to_value(&resp)?;
        return Ok(Some(result));
    }

    #[derive(QueryableByName, Serialize)]
    struct Record {
        #[sql_type = "Int4"]
        title_id: Id,
        #[sql_type = "Text"]
        title: String,
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int8"]
        row: i64,
        #[sql_type = "Text"]
        content: String,
    }

    const MANDELA_TYPE: i8 = 0;
    const COMMENT_TYPE: i8 = 1;

    let mandela_title = "(CASE WHEN title_mode = 0 THEN title ELSE what || ': ' || before || ' / ' || after END) AS title";

    let (mut sql_content, sql_count) = if req.type_ == MANDELA_TYPE {
        let source = "title || ' ' || what || ' ' || before || ' ' || after || ' ' || description";

        (
            format!(
                "SELECT 0 AS id, 0::Int8 AS row, {0}, id AS title_id,
                ts_headline({1}, plainto_tsquery('russian', $1)) AS content
            FROM mandels
            WHERE to_tsvector('russian', {1}) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', {1}), plainto_tsquery('russian', $1)) DESC",
                mandela_title, source
            ),
            format!(
                "SELECT count(*)
            FROM mandels
            WHERE to_tsvector('russian', {}) @@ plainto_tsquery('russian', $1)",
                source
            ),
        )
    } else if req.type_ == COMMENT_TYPE {
        (format!(
            "SELECT c.id,
                (SELECT row FROM (SELECT id, row_number() OVER (PARTITION BY mandela_id ORDER BY id ASC) AS row
                    FROM comments WHERE mandela_id = c.mandela_id) AS x WHERE x.id = c.id) AS row,
                m.id AS title_id,
                {},
                ts_headline('russian', c.message, plainto_tsquery($1)) AS content
            FROM comments AS c
            JOIN mandels AS m ON m.id = c.mandela_id
            WHERE to_tsvector('russian', c.message) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', c.message), plainto_tsquery('russian', $1)) DESC", mandela_title),

        "SELECT count(*)
        FROM comments AS c
        JOIN mandels AS m ON m.id = c.mandela_id
        WHERE to_tsvector('russian', c.message) @@ plainto_tsquery('russian', $1)".to_string())
    } else {
        // forum post
        (format!(
            "SELECT fp.id,
                (SELECT row FROM (SELECT id, row_number() OVER (PARTITION BY topic_id ORDER BY id ASC) AS row
                    FROM forum_posts WHERE topic_id = fp.topic_id) AS x WHERE x.id = fp.id) AS row,
                ft.id AS title_id, ft.name AS title,
                ts_headline('russian', fp.post, plainto_tsquery($1)) AS content
            FROM forum_posts AS fp
            JOIN forum_topics AS ft ON ft.id = fp.topic_id
            WHERE to_tsvector('russian', fp.post) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', fp.post), plainto_tsquery('russian', $1)) DESC"),

        "SELECT count(*)
        FROM forum_posts AS fp
        JOIN forum_topics AS ft ON ft.id = fp.topic_id
        WHERE to_tsvector('russian', fp.post) @@ plainto_tsquery('russian', $1)".to_string())
    };

    sql_content = sql_content + " LIMIT $2 OFFSET $3";

    let records = sql_query(sql_content)
        .bind::<Text, _>(&req.text)
        .bind::<Int8, _>(req.limit)
        .bind::<Int8, _>(req.offset)
        .load::<Record>(&data.db.conn)?;

    #[derive(QueryableByName)]
    struct TotalCount {
        #[sql_type = "Int8"]
        count: i64,
    }

    let total_count = sql_query(sql_count)
        .bind::<Text, _>(&req.text)
        .load::<TotalCount>(&data.db.conn)?;

    let resp = Resp {
        records: records,
        total_count: total_count[0].count,
    };
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

use super::*;
use crate::types::Id;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::sql_types::Int8;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;

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

    if req.text.is_empty() {
        let data = r#"[]"#;
        let result = serde_json::from_str(data)?;
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

    let mut sql = if req.type_ == MANDELA_TYPE {
        let source = "title || ' ' || what || ' ' || before || ' ' || after || ' ' || description";

        format!(
            "SELECT 0 AS id, 0::Int8 AS row, {0}, id AS title_id,
                ts_headline({1}, plainto_tsquery('russian', $1)) AS content
            FROM mandels
            WHERE to_tsvector('russian', {1}) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', {1}), plainto_tsquery('russian', $1)) DESC",
            mandela_title, source
        )
    } else if req.type_ == COMMENT_TYPE {
        format!(
            "SELECT c.id,
                (SELECT row FROM (SELECT id, row_number() OVER (PARTITION BY mandela_id ORDER BY id ASC) AS row
                    FROM comments WHERE mandela_id = c.mandela_id) AS x WHERE x.id = c.id) AS row,
                m.id AS title_id,
                {},
                ts_headline('russian', c.message, plainto_tsquery($1)) AS content
            FROM comments AS c
            JOIN mandels AS m ON m.id = c.mandela_id
            WHERE to_tsvector('russian', c.message) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', c.message), plainto_tsquery('russian', $1)) DESC", mandela_title)
    } else {
        // forum post
        format!(
            "SELECT fp.id,
                (SELECT row FROM (SELECT id, row_number() OVER (PARTITION BY topic_id ORDER BY id ASC) AS row
                    FROM forum_posts WHERE topic_id = fp.topic_id) AS x WHERE x.id = fp.id) AS row,
                ft.id AS title_id, ft.name AS title,
                ts_headline('russian', fp.post, plainto_tsquery($1)) AS content
            FROM forum_posts AS fp
            JOIN forum_topics AS ft ON ft.id = fp.topic_id
            WHERE to_tsvector('russian', fp.post) @@ plainto_tsquery('russian', $1)
            ORDER BY ts_rank(to_tsvector('russian', fp.post), plainto_tsquery('russian', $1)) DESC")
    };

    sql = sql + " LIMIT $2 OFFSET $3";

    let records = sql_query(sql)
        .bind::<Text, _>(req.text)
        .bind::<Int8, _>(req.limit)
        .bind::<Int8, _>(req.offset)
        .load::<Record>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        records: Vec<Record>,
    };

    let resp = Resp { records: records };
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

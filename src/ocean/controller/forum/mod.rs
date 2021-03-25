use crate::controller::*;
use crate::types::Id;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::sql_types::Int8;
use diesel::sql_types::Text;
use diesel::sql_types::Timestamptz;
use serde::{Deserialize, Serialize};

pub mod category;
pub mod post;
pub mod section;
pub mod topic;

#[derive(QueryableByName, Serialize)]
pub struct Topic {
    #[sql_type = "Int4"]
    id: Id,
    #[sql_type = "Text"]
    name: String,
    #[sql_type = "Text"]
    post: String,
    #[sql_type = "Int4"]
    post_id: Id,
    #[sql_type = "Timestamptz"]
    post_create_ts: NaiveDateTime,
    #[sql_type = "Int4"]
    user_id: Id,
    #[sql_type = "Text"]
    user_name: String,
    #[sql_type = "Int8"]
    post_count: i64,
}

// forum.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;

    #[derive(Queryable, Serialize)]
    pub struct Category {
        id: Id,
        name: String,
    }

    let categories = forum_categories::table
        .select((forum_categories::id, forum_categories::name))
        .order(forum_categories::order_index.asc())
        .load::<Category>(&data.db.conn)?;

    let sections = section::get_sections(&data.db, None)?;

    #[derive(Serialize)]
    struct Resp {
        categories: Vec<Category>,
        sections: Vec<section::Section>,
    }

    let resp = Resp {
        categories: categories,
        sections: sections,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.getNew
pub fn get_new(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        offset: i32,
        limit: i32,
    }

    let req: Req = data.params()?;

    use diesel::prelude::*;

    use crate::model::schema::forum_topics::dsl::*;

    let list = new_topics(&data.db, req.limit, req.offset)?;

    let topic_count: i64 = forum_topics
        .filter(last_post_create_ts.is_not_null())
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        topic_count: i64,
        topics: Vec<Topic>,
    }

    let resp = Resp {
        topic_count: topic_count,
        topics: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

pub fn new_topics(
    db: &db::Db,
    limit: i32,
    offset: i32,
) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
    let result = diesel::dsl::sql_query("
    SELECT ft.id, ft.name, fp.post, fp.id AS post_id, fp.create_ts AS post_create_ts, u.id AS user_id, u.name AS user_name,
        (SELECT count(*) FROM forum_posts WHERE topic_id = ft.id) AS post_count
    FROM forum_topics AS ft
        INNER JOIN forum_posts AS fp ON fp.id = ft.last_post_id
        INNER JOIN users AS u ON u.id = fp.user_id
    WHERE last_post_create_ts IS NOT NULL
    ORDER BY last_post_create_ts DESC
    LIMIT $1
    OFFSET $2")
    .bind::<Int4, _>(limit)
    .bind::<Int4, _>(offset)
    .load::<Topic>(&db.conn)?;

    Ok(result)
}

use crate::controller::*;
use crate::types::Id;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub mod category;
pub mod post;
pub mod section;
pub mod topic;

// forum.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_sections;

    #[derive(Queryable, Serialize)]
    pub struct Category {
        id: Id,
        name: String,
    }

    let categories = forum_categories::table
        .select((forum_categories::id, forum_categories::name))
        .order(forum_categories::order_index.asc())
        .load::<Category>(&data.db.conn)?;

    #[derive(Queryable, Serialize)]
    pub struct Section {
        id: Id,
        name: String,
        category_id: Id,
    }

    let sections = forum_sections::table
        .select((
            forum_sections::id,
            forum_sections::name,
            forum_sections::category_id,
        ))
        .order(forum_sections::order_index.asc())
        .load::<Section>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        categories: Vec<Category>,
        sections: Vec<Section>,
    };

    let resp = serde_json::to_value(&Resp {
        categories: categories,
        sections: sections,
    })?;

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

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use diesel::prelude::*;
    use diesel::sql_types::Int4;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Text;
    use diesel::sql_types::Timestamptz;

    #[derive(QueryableByName, Serialize)]
    struct Topic {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Text"]
        name: String,
        #[sql_type = "Text"]
        post: String,
        #[sql_type = "Timestamptz"]
        post_create_ts: NaiveDateTime,
        #[sql_type = "Int4"]
        user_id: Id,
        #[sql_type = "Text"]
        user_name: String,
        #[sql_type = "Int8"]
        post_count: i64,
    }

    let list = diesel::dsl::sql_query("
        SELECT ft.id, ft.name, fp.post, fp.create_ts AS post_create_ts, u.id AS user_id, u.name AS user_name,
            (SELECT count(*) FROM forum_posts WHERE topic_id = ft.id) AS post_count
        FROM forum_topics AS ft
            INNER JOIN forum_posts AS fp ON fp.id = ft.last_post_id
            INNER JOIN users AS u ON u.id = fp.user_id
        WHERE last_post_create_ts IS NOT NULL
        ORDER BY last_post_create_ts DESC
        LIMIT $1
        OFFSET $2"
    )
    .bind::<Int4, _>(req.limit)
    .bind::<Int4, _>(req.offset)
    .load::<Topic>(&data.db.conn)?;

    use crate::model::schema::forum_topics::dsl::*;

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

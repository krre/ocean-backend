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
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    struct Topic {
        id: Id,
        name: String,
        post: String,
        post_create_ts: NaiveDateTime,
        user_id: Id,
        user_name: Option<String>,
    }

    use crate::model::schema::forum_posts;
    use crate::model::schema::forum_posts::dsl::*;
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    let list = forum_topics
        .inner_join(forum_posts.on(forum_posts::id.nullable().eq(forum_topics::last_post_id)))
        .inner_join(users.on(users::id.eq(forum_posts::user_id)))
        .select((
            forum_topics::id,
            forum_topics::name,
            forum_posts::post,
            forum_posts::create_ts,
            users::id,
            users::name.nullable(),
        ))
        .filter(last_post_create_ts.is_not_null())
        .order(last_post_create_ts.desc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<Topic>(&data.db.conn)?;

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

use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

// forum.topic.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        section_id: Id,
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    struct Topic {
        id: Id,
        user_id: Id,
        user_name: Option<String>,
        name: String,
        create_ts: NaiveDateTime,
    }

    #[derive(Serialize)]
    struct Resp {
        section_name: String,
        topic_count: i64,
        topics: Vec<Topic>,
    }

    let section_name = forum_sections::table
        .select(forum_sections::name)
        .filter(forum_sections::id.eq(req.section_id))
        .first::<String>(&data.db.conn)?;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    let list = forum_topics
        .inner_join(users)
        .select((
            forum_topics::id,
            users::id,
            users::name,
            forum_topics::name,
            forum_topics::create_ts,
        ))
        .filter(section_id.eq(req.section_id))
        .order(forum_topics::create_ts.desc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<Topic>(&data.db.conn)?;

    let topic_count: i64 = forum_topics
        .filter(section_id.eq(req.section_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    let resp = Resp {
        section_name: section_name,
        topic_count: topic_count,
        topics: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.topic.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_topics::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    pub struct ForumTopic {
        section_id: Id,
        name: String,
    }

    let forum_topic = forum_topics
        .select((section_id, name))
        .filter(id.eq(req.id))
        .first::<ForumTopic>(&data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_topic)?;
    Ok(Some(result))
}

// forum.topic.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        section_id: Id,
        name: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Insertable)]
    #[table_name = "forum_topics"]
    struct NewForumTopic {
        section_id: Id,
        user_id: Id,
        name: String,
    }

    let new_forum_topic = NewForumTopic {
        section_id: req.section_id,
        user_id: data.user.id,
        name: req.name,
    };

    let topic_id = diesel::insert_into(forum_topics)
        .values(&new_forum_topic)
        .returning(id)
        .get_result::<Id>(&data.db.conn)?;

    let result = json!({ "id": topic_id });
    Ok(Some(result))
}

// forum.topic.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        name: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(AsChangeset)]
    #[table_name = "forum_topics"]
    pub struct UpdateForumTopic {
        pub name: String,
        pub update_ts: NaiveDateTime,
    }

    let update_forum_topic = UpdateForumTopic {
        name: req.name,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(forum_topics.filter(id.eq(req.id)))
        .set(&update_forum_topic)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.topic.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_topics::dsl::*;
    let forum_topic_id = data.params.unwrap()["id"].as_i64().unwrap() as Id;

    diesel::delete(forum_topics.filter(id.eq(forum_topic_id))).execute(&data.db.conn)?;
    Ok(None)
}

use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

pub const COMMON_TOPIC_TYPE: i16 = 0;
pub const POLL_TOPIC_TYPE: i16 = 1;

// forum.topic.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        section_id: Id,
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable)]
    struct SectionMeta {
        category_id: Id,
        category_name: String,
        section_name: String,
    };

    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;

    let section_meta = forum_sections::table
        .inner_join(
            forum_categories::table.on(forum_categories::id.eq(forum_sections::category_id)),
        )
        .select((
            forum_categories::id,
            forum_categories::name,
            forum_sections::name,
        ))
        .filter(forum_sections::id.eq(req.section_id))
        .first::<SectionMeta>(&data.db.conn)?;

    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Queryable, Serialize)]
    struct Topic {
        id: Id,
        user_id: Id,
        user_name: String,
        name: String,
        #[serde(rename(serialize = "type"))]
        type_: i16,
        create_ts: NaiveDateTime,
    }

    let list = forum_topics
        .inner_join(users)
        .select((
            forum_topics::id,
            users::id,
            users::name,
            forum_topics::name,
            type_,
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

    #[derive(Serialize)]
    struct Resp {
        category_id: Id,
        category_name: String,
        section_name: String,
        topic_count: i64,
        topics: Vec<Topic>,
    }

    let resp = Resp {
        category_id: section_meta.category_id,
        category_name: section_meta.category_name,
        section_name: section_meta.section_name,
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
        user_id: Id,
        section_id: Id,
        name: String,
    }

    let forum_topic = forum_topics
        .select((user_id, section_id, name))
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
        #[serde(rename(deserialize = "type"))]
        topic_type: i16,
        poll_answers: Option<Vec<String>>,
        poll_answer_selection: Option<i16>,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Insertable)]
    #[table_name = "forum_topics"]
    struct NewForumTopic {
        section_id: Id,
        user_id: Id,
        name: String,
        type_: i16,
        poll_selection_type: Option<i16>,
    }

    let new_forum_topic = NewForumTopic {
        section_id: req.section_id,
        user_id: data.user.id,
        name: req.name,
        type_: req.topic_type,
        poll_selection_type: req.poll_answer_selection,
    };

    let mut topic_id: Id = 0;
    let conn = data.db.conn;

    let topic_type = req.topic_type;
    // Temporary array to shut up borrow checker while passing to transaction closure
    let mut answers: Vec<String> = Vec::new();

    if topic_type == POLL_TOPIC_TYPE {
        answers = req.poll_answers.unwrap().clone();
    }

    conn.transaction::<_, diesel::result::Error, _>(|| {
        topic_id = diesel::insert_into(forum_topics)
            .values(&new_forum_topic)
            .returning(id)
            .get_result::<Id>(&conn)?;

        if topic_type == POLL_TOPIC_TYPE {
            use crate::model::schema::forum_poll_answers;

            for answer in answers {
                diesel::insert_into(forum_poll_answers::table)
                    .values((
                        forum_poll_answers::topic_id.eq(topic_id),
                        forum_poll_answers::answer.eq(answer),
                    ))
                    .execute(&conn)?;
            }
        }

        Ok(())
    })?;

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

pub fn update_last_post(
    db: &db::Db,
    id: Id,
    post_id: Option<Id>,
    post_create_ts: Option<NaiveDateTime>,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::model::schema::forum_topics;

    #[derive(AsChangeset)]
    #[table_name = "forum_topics"]
    #[changeset_options(treat_none_as_null = "true")]
    pub struct UpdateForumTopic {
        last_post_id: Option<Id>,
        last_post_create_ts: Option<NaiveDateTime>,
    }

    let update_forum_topic = UpdateForumTopic {
        last_post_id: post_id,
        last_post_create_ts: post_create_ts,
    };

    diesel::update(forum_topics::table.filter(forum_topics::id.eq(id)))
        .set(&update_forum_topic)
        .execute(&db.conn)?;

    Ok(())
}

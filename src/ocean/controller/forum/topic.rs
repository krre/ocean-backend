use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Bool;
use diesel::sql_types::Int2;
use diesel::sql_types::Int4;
use diesel::sql_types::Int8;
use diesel::sql_types::Nullable;
use diesel::sql_types::Text;
use diesel::sql_types::Timestamptz;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

pub const COMMON_TOPIC_TYPE: i16 = 0;
pub const POLL_TOPIC_TYPE: i16 = 1;

#[derive(Serialize, QueryableByName)]
pub struct Poll {
    #[sql_type = "Int4"]
    id: Id,
    #[sql_type = "Text"]
    answer: String,
    #[sql_type = "Int8"]
    count: i64,
    #[sql_type = "Bool"]
    voted: bool,
}

// forum.topic.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        section_id: Id,
        offset: i64,
        limit: i64,
    }

    let req: Req = data.params()?;

    #[derive(Queryable)]
    struct SectionMeta {
        category_id: Id,
        category_name: String,
        section_name: String,
    };

    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_sections;
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

    #[derive(QueryableByName, Serialize)]
    struct Topic {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int4"]
        user_id: Id,
        #[sql_type = "Text"]
        user_name: String,
        #[sql_type = "Text"]
        name: String,
        #[serde(rename(serialize = "type"))]
        #[sql_type = "Int2"]
        type_: i16,
        #[sql_type = "Timestamptz"]
        create_ts: NaiveDateTime,
        #[sql_type = "Nullable<Int4>"]
        last_post_id: Option<Id>,
        #[sql_type = "Nullable<Timestamptz>"]
        last_post_create_ts: Option<NaiveDateTime>,
        #[sql_type = "Int8"]
        post_count: i64,
    }

    let topics = diesel::dsl::sql_query(
        "
    SELECT ft.id, ft.user_id, ft.last_post_id, ft.last_post_create_ts, ft.name, ft.type AS type_, ft.create_ts, u.name AS user_name,
	    (SELECT COUNT(*) FROM forum_posts WHERE topic_id = ft.id) AS post_count
    FROM forum_topics AS ft
        JOIN users AS u ON u.id = ft.user_id
    WHERE section_id = $1
    ORDER BY ft.last_post_create_ts DESC
    OFFSET $2
    LIMIT $3",
    )
    .bind::<Int4, _>(req.section_id)
    .bind::<Int8, _>(req.offset)
    .bind::<Int8, _>(req.limit)
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
        topics: topics,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.topic.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_topics::dsl::*;

    let req: RequestId = data.params()?;

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

    #[derive(Deserialize, Clone)]
    struct Req {
        section_id: Id,
        name: String,
        #[serde(rename(deserialize = "type"))]
        topic_type: i16,
        poll_answers: Option<Vec<String>>,
        poll_answer_selection: Option<i16>,
    }

    let req: Req = data.params()?;

    #[derive(Insertable)]
    #[table_name = "forum_topics"]
    struct NewForumTopic {
        section_id: Id,
        user_id: Id,
        name: String,
        type_: i16,
        poll_selection_type: Option<i16>,
    }

    let r = req.clone();

    let new_forum_topic = NewForumTopic {
        section_id: req.section_id,
        user_id: data.user.id,
        name: req.name,
        type_: req.topic_type,
        poll_selection_type: req.poll_answer_selection,
    };

    let mut topic_id: Id = 0;
    let conn = data.db.conn;

    conn.transaction::<_, diesel::result::Error, _>(|| {
        topic_id = diesel::insert_into(forum_topics)
            .values(&new_forum_topic)
            .returning(id)
            .get_result::<Id>(&conn)?;

        if r.topic_type == POLL_TOPIC_TYPE {
            use crate::model::schema::forum_poll_answers;

            for answer in r.poll_answers.unwrap() {
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

    let req: Req = data.params()?;

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
    let req: RequestId = data.params()?;

    use crate::model::schema::forum_topics::dsl::*;
    diesel::delete(forum_topics.filter(id.eq(req.id))).execute(&data.db.conn)?;
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

// forum.topic.vote
pub fn vote(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Id,
        votes: Vec<Id>,
    }

    let req: Req = data.params()?;
    let conn = &data.db.conn;
    let poll_user_id = data.user.id;
    let poll_topic_id = req.id;

    conn.transaction::<_, diesel::result::Error, _>(|| {
        use crate::model::schema::forum_poll_votes;
        use crate::model::schema::forum_poll_votes::dsl::*;
        diesel::delete(
            forum_poll_votes.filter(topic_id.eq(poll_topic_id).and(user_id.eq(poll_user_id))),
        )
        .execute(conn)?;
        #[derive(Insertable)]
        #[table_name = "forum_poll_votes"]
        struct NewVote {
            topic_id: Id,
            answer_id: Id,
            user_id: Id,
        };
        for vote in req.votes {
            let new_vote = NewVote {
                topic_id: poll_topic_id,
                user_id: poll_user_id,
                answer_id: vote,
            };
            diesel::insert_into(forum_poll_votes)
                .values(&new_vote)
                .execute(conn)?;
        }

        Ok(())
    })?;

    let poll = get_poll(&data.db, poll_topic_id, poll_user_id);

    #[derive(Serialize)]
    struct Resp {
        poll: Vec<Poll>,
    }

    let resp = Resp { poll: poll };
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

pub fn get_poll(db: &db::Db, topic_id: Id, user_id: Id) -> Vec<Poll> {
    diesel::dsl::sql_query(
        "SELECT fpa.id, answer, COUNT(fpv.*),
            (SELECT true AS voted FROM forum_poll_votes WHERE answer_id = fpa.id AND user_id = $2) AS voted
        FROM forum_poll_answers AS fpa
            LEFT JOIN forum_poll_votes AS fpv ON fpv.answer_id = fpa.id
        WHERE fpa.topic_id = $1
        GROUP BY fpa.id
        ORDER BY fpa.id ASC",
    )
    .bind::<Int4, _>(topic_id)
    .bind::<Int4, _>(user_id)
    .load::<Poll>(&db.conn)
    .unwrap()
}

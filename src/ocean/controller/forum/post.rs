use crate::controller::forum::topic;
use crate::controller::*;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Int2, Int4, Int8, Nullable, Text, Timestamptz};
use serde::{Deserialize, Serialize};

// forum.post.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        topic_id: Id,
        offset: i32,
        limit: i32,
    }

    let req: Req = data.params()?;

    #[derive(Queryable)]
    struct TopicMeta {
        category_id: Id,
        category_name: String,
        section_id: Id,
        section_name: String,
        topic_name: String,
        topic_type: i16,
        topic_user_id: Id,
        poll_selection_type: Option<i16>,
    }

    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_posts::dsl::*;
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_topics;
    use crate::model::schema::forum_topics::dsl::*;

    let topic_meta = forum_topics
        .inner_join(forum_sections::table.on(forum_sections::id.eq(forum_topics::section_id)))
        .inner_join(
            forum_categories::table.on(forum_categories::id.eq(forum_sections::category_id)),
        )
        .select((
            forum_categories::id,
            forum_categories::name,
            forum_sections::id,
            forum_sections::name,
            forum_topics::name,
            forum_topics::type_,
            forum_topics::user_id,
            forum_topics::poll_selection_type,
        ))
        .filter(forum_topics::id.eq(req.topic_id))
        .first::<TopicMeta>(&data.db.conn)?;

    #[derive(QueryableByName, Serialize)]
    struct Post {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int4"]
        user_id: Id,
        #[sql_type = "Text"]
        user_name: String,
        #[sql_type = "Text"]
        post: String,
        #[sql_type = "Int8"]
        like_count: i64,
        #[sql_type = "Int8"]
        dislike_count: i64,
        #[sql_type = "Nullable<Int2>"]
        like: Option<i16>,
        #[sql_type = "Timestamptz"]
        create_ts: NaiveDateTime,
    }

    let list = diesel::sql_query(
        "SELECT fp.id, u.id AS user_id, u.name AS user_name, post, l.value AS like, fp.create_ts,
            (SELECT count(*) FROM likes WHERE post_id = fp.id AND value = 0) AS like_count,
            (SELECT count(*) FROM likes WHERE post_id = fp.id AND value = 1) AS dislike_count
        FROM forum_posts AS fp
            JOIN users AS u ON u.id = fp.user_id
            LEFT JOIN likes AS l ON l.post_id = fp.id AND l.user_id = $1
        WHERE topic_id = $2
        ORDER BY fp.id ASC
        OFFSET $3
        LIMIT $4",
    )
    .bind::<Int4, _>(data.user.id)
    .bind::<Int4, _>(req.topic_id)
    .bind::<Int4, _>(req.offset)
    .bind::<Int4, _>(req.limit)
    .load::<Post>(&data.db.conn)?;

    let post_count: i64 = forum_posts
        .filter(topic_id.eq(req.topic_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    let mut poll: Option<Vec<topic::Poll>> = None;

    if topic_meta.topic_type == topic::POLL_TOPIC_TYPE {
        let answers = topic::get_poll(&data.db, req.topic_id, data.user.id);
        poll = Some(answers);
    }

    #[derive(Serialize)]
    struct Resp {
        category_id: Id,
        category_name: String,
        section_id: Id,
        section_name: String,
        topic_name: String,
        topic_type: i16,
        topic_user_id: Id,
        poll_selection_type: Option<i16>,
        poll: Option<Vec<topic::Poll>>,
        post_count: i64,
        posts: Vec<Post>,
    }

    let resp = Resp {
        category_id: topic_meta.category_id,
        category_name: topic_meta.category_name,
        section_id: topic_meta.section_id,
        section_name: topic_meta.section_name,
        topic_name: topic_meta.topic_name,
        topic_type: topic_meta.topic_type,
        topic_user_id: topic_meta.topic_user_id,
        poll_selection_type: topic_meta.poll_selection_type,
        poll,
        post_count,
        posts: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.post.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_posts::dsl::*;

    let req: RequestId = data.params()?;

    #[derive(Queryable, Serialize)]
    pub struct ForumPost {
        topic_id: Id,
        post: String,
    }

    let forum_post = forum_posts
        .select((topic_id, post))
        .filter(id.eq(req.id))
        .first::<ForumPost>(&data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_post)?;
    Ok(Some(result))
}

// forum.post.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_posts;
    use crate::model::schema::forum_posts::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        topic_id: Id,
        post: String,
    }

    let req: Req = data.params()?;

    #[derive(Insertable)]
    #[table_name = "forum_posts"]
    struct NewForumPost<'a> {
        topic_id: Id,
        user_id: Id,
        post: &'a str,
    }

    let new_forum_post = NewForumPost {
        topic_id: req.topic_id,
        user_id: data.user.id,
        post: &req.post,
    };

    let (post_id, post_create_ts) = diesel::insert_into(forum_posts)
        .values(&new_forum_post)
        .returning((forum_posts::id, forum_posts::create_ts))
        .get_result::<(Id, NaiveDateTime)>(&data.db.conn)?;

    topic::update_last_post(&data.db, req.topic_id, Some(post_id), Some(post_create_ts))?;

    use crate::model::schema::forum_topics;
    use crate::model::schema::users;

    let topic_name = forum_topics::table
        .select(forum_topics::name)
        .filter(forum_topics::id.eq(req.topic_id))
        .first::<String>(&data.db.conn)?;

    let topic_user_name = users::table
        .select(users::name)
        .filter(users::id.eq(data.user.id))
        .first::<String>(&data.db.conn)?;

    let topic_title = format!(
        "<a href='{}/forum/topic/{}'>{}</a>",
        config::CONFIG.frontend.domen,
        req.topic_id,
        topic_name
    );

    let post_message = format!(
        "Форум
{}
{}

{}",
        topic_title, topic_user_name, req.post
    );

    telegram_bot::send_admin_message(&post_message);

    let resp = ResponseId { id: post_id };
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.post.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_posts;
    use crate::model::schema::forum_posts::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        post: String,
    }

    let req: Req = data.params()?;

    #[derive(AsChangeset)]
    #[table_name = "forum_posts"]
    pub struct UpdateForumPost {
        pub post: String,
        pub update_ts: NaiveDateTime,
    }

    let update_forum_post = UpdateForumPost {
        post: req.post,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(forum_posts.filter(id.eq(req.id)))
        .set(&update_forum_post)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.post.delete
pub fn delete(data: RequestData) -> RequestResult {
    let req: RequestId = data.params()?;

    use crate::model::schema::forum_posts;

    let topic_id = forum_posts::table
        .select(forum_posts::topic_id)
        .filter(forum_posts::id.eq(req.id))
        .first::<Id>(&data.db.conn)?;

    #[derive(Queryable, Serialize)]
    pub struct ForumPost {
        id: Id,
        create_ts: NaiveDateTime,
    }

    let forum_post = forum_posts::table
        .select((forum_posts::id, forum_posts::create_ts))
        .filter(forum_posts::topic_id.eq(topic_id))
        .order(forum_posts::id.desc())
        .offset(1)
        .first::<ForumPost>(&data.db.conn)
        .optional()?;

    let mut prev_post_id: Option<Id> = None;
    let mut prev_post_create_ts: Option<NaiveDateTime> = None;

    if let Some(fp) = forum_post {
        prev_post_id = Some(fp.id);
        prev_post_create_ts = Some(fp.create_ts);
    }

    topic::update_last_post(&data.db, topic_id, prev_post_id, prev_post_create_ts)?;
    diesel::delete(forum_posts::table.filter(forum_posts::id.eq(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

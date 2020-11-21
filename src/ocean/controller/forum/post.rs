use crate::controller::forum::topic;
use crate::controller::*;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

// forum.post.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_posts;
    use crate::model::schema::forum_posts::dsl::*;
    use crate::model::schema::forum_topics;

    #[derive(Deserialize)]
    struct Req {
        topic_id: Id,
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    struct Post {
        id: Id,
        user_id: Id,
        user_name: Option<String>,
        post: String,
        create_ts: NaiveDateTime,
    }

    #[derive(Serialize)]
    struct Resp {
        topic_name: String,
        post_count: i64,
        posts: Vec<Post>,
    }

    let topic_name = forum_topics::table
        .select(forum_topics::name)
        .filter(forum_topics::id.eq(req.topic_id))
        .first::<String>(&data.db.conn)?;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    let list = forum_posts
        .inner_join(users)
        .select((
            forum_posts::id,
            users::id,
            users::name,
            forum_posts::post,
            forum_posts::create_ts,
        ))
        .filter(topic_id.eq(req.topic_id))
        .order(forum_posts::id.asc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<Post>(&data.db.conn)?;

    let post_count: i64 = forum_posts
        .filter(topic_id.eq(req.topic_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    let resp = Resp {
        topic_name: topic_name,
        post_count: post_count,
        posts: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.topic.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_posts::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

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

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

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
        .first::<Option<String>>(&data.db.conn)?;

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
        topic_title,
        topic_user_name.unwrap(),
        req.post
    );

    telegram_bot::send_admin_message(post_message);

    let result = json!({ "id": post_id });
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

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

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
    use crate::model::schema::forum_posts;

    let post_id = data.params.unwrap()["id"].as_i64().unwrap() as Id;

    let topic_id = forum_posts::table
        .select(forum_posts::topic_id)
        .filter(forum_posts::id.eq(post_id))
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
    diesel::delete(forum_posts::table.filter(forum_posts::id.eq(post_id)))
        .execute(&data.db.conn)?;
    Ok(None)
}

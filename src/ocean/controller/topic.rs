use super::*;
use crate::model::date_serializer;
use crate::model::topic;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

// topic.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        title: String,
        description: String,
        images: serde_json::Value,
        videos: serde_json::Value,
        links: serde_json::Value,
        user_id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::topics::dsl::*;

    let new_topic = topic::NewTopic {
        title: &req.title,
        description: &req.description,
        images: Some(req.images),
        videos: Some(req.videos),
        links: Some(req.links),
        user_id: req.user_id,
    };
    let topic_id = diesel::insert_into(topics)
        .values(&new_topic)
        .returning(id)
        .get_result::<i32>(&data.db.conn)?;

    let result = json!({ "id": topic_id });

    Ok(Some(result))
}

// topic.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::topics;
    use crate::model::schema::topics::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        id: i32,
        title: String,
        description: String,
        images: serde_json::Value,
        videos: serde_json::Value,
        links: serde_json::Value,
        user_id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let update_topic = topic::UpdateTopic {
        title: req.title,
        description: req.description,
        images: Some(req.images),
        videos: Some(req.videos),
        links: Some(req.links),
        user_id: req.user_id,
    };

    diesel::update(topics.filter(topics::id.eq(req.id)))
        .set(&update_topic)
        .execute(&data.db.conn)?;

    Ok(None)
}

// topic.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::topics::dsl::*;

    let topic_id = data.params.unwrap()["id"].as_i64().unwrap() as i32;
    let record = topics
        .filter(id.eq(topic_id))
        .limit(1)
        .load::<topic::Topic>(&data.db.conn)?;

    let result = serde_json::to_value(&record)?;
    Ok(Some(result))
}

// topic.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::topics;
    use crate::model::schema::topics::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    struct TopicResp {
        id: i32,
        title: String,
        #[serde(with = "date_serializer")]
        create_ts: NaiveDateTime,
        name: Option<String>,
        user_id: i32,
    }

    let list = topics
        .inner_join(users)
        .select((topics::id, title, topics::create_ts, users::name, users::id))
        .order(topics::id.desc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<TopicResp>(&data.db.conn)?;

    let total_count: i64 = topics
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        topics: Vec<TopicResp>,
    };

    let resp = serde_json::to_value(&Resp {
        total_count: total_count,
        topics: list,
    })?;

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// topic.delete
pub fn delete(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Vec<i32>,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::topics::dsl::*;

    diesel::delete(topics.filter(id.eq_any(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

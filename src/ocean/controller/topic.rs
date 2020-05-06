use super::*;
use crate::model::topic;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

// topic.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        title: String,
        description: String,
        user_id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::topics::dsl::*;

    let new_topic = topic::NewTopic {
        title: &req.title,
        description: &req.description,
        user_id: req.user_id,
    };
    let topic_id = diesel::insert_into(topics)
        .values(&new_topic)
        .returning(id)
        .get_result::<i32>(&data.db.conn)?;

    let result = json!({ "id": topic_id });

    Ok(Some(result))
}

// topic.get
pub fn get(data: RequestData) -> RequestResult {
    use crate::model::schema::topics::dsl::*;

    let list = {
        if let Some(p) = data.params {
            let topic_id = p["id"].as_i64().unwrap() as i32;
            topics
                .filter(id.eq(topic_id))
                .limit(1)
                .load::<topic::Topic>(&data.db.conn)?
        } else {
            topics.load::<topic::Topic>(&data.db.conn)?
        }
    };

    let result = serde_json::to_value(&list)?;
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

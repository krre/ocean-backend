use super::Controller;
use crate::api;
use crate::db;
use crate::model::topic;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use serde_json::json;

pub struct Topic {}

#[derive(Deserialize)]
struct CreateRequest {
    title: String,
    description: String,
    user_id: i32,
}

#[derive(Deserialize)]
struct DeleteRequest {
    id: Vec<i32>,
}

impl Topic {
    fn create(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        let request: CreateRequest = serde_json::from_value(params.unwrap()).unwrap();

        use crate::model::schema::topics::dsl::*;

        let new_topic = topic::NewTopic {
            title: &request.title,
            description: &request.description,
            user_id: request.user_id,
        };
        let result: topic::Topic = diesel::insert_into(topics)
            .values(&new_topic)
            // .returning(id)
            .get_result(&db.conn)
            .unwrap();

        let result = json!({
            "id": result.id
        });

        Ok(Some(result))
    }

    fn get(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        use crate::model::schema::topics::dsl::*;

        let list = {
            if let Some(p) = params {
                let topic_id = p["id"].as_i64().unwrap() as i32;
                topics
                    .filter(id.eq(topic_id))
                    .limit(1)
                    .load::<topic::Topic>(&db.conn)
                    .unwrap()
            } else {
                topics.load::<topic::Topic>(&db.conn).unwrap()
            }
        };

        let result = serde_json::to_value(&list).unwrap();
        Ok(Some(result))
    }

    fn remove(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        use crate::model::schema::topics::dsl::*;

        let delete_request: DeleteRequest = serde_json::from_value(params.unwrap()).unwrap();

        diesel::delete(topics.filter(id.eq_any(delete_request.id)))
            .execute(&db.conn)
            .unwrap();
        Ok(None)
    }
}

impl Controller for Topic {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        match method {
            "create" => self.create(db, params),
            "get" => self.get(db, params),
            "remove" => self.remove(db, params),
            _ => Err(api::error::Error::new(
                api::error::METHOD_NOT_FOUND,
                Some(method.to_string()),
            )),
        }
    }
}

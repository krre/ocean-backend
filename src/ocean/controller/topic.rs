use super::Controller;
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
}

impl Topic {
    fn create(&self, db: &db::Db, params: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let request: CreateRequest = serde_json::from_value(params.unwrap()).unwrap();

        use crate::model::schema::topics::dsl::*;

        let new_topic = topic::NewTopic {
            title: &request.title,
            description: &request.description,
        };
        let result: topic::Topic = diesel::insert_into(topics)
            .values(&new_topic)
            // .returning(id)
            .get_result(&db.conn)
            .unwrap();

        let result = json!({
            "id": result.id
        });

        Some(result)
    }

    fn get(&self, db: &db::Db, _params: Option<serde_json::Value>) -> Option<serde_json::Value> {
        use crate::model::schema::topics::dsl::*;

        let list = topics.load::<topic::Topic>(&db.conn).unwrap();
        let result = serde_json::to_value(&list).unwrap();
        Some(result)
    }
}

impl Controller for Topic {
    fn new() -> Topic {
        Topic {}
    }

    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Option<serde_json::Value> {
        match method {
            "create" => self.create(db, params),
            "get" => self.get(db, params),
            _ => {
                println!("method {} not found", method);
                None
            }
        }
    }
}

use super::Controller;
use crate::api;
use crate::db;
use crate::model::topic;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use serde_json::json;
use std::error::Error;

pub struct Topic;

impl Topic {
    fn create(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct Req {
            title: String,
            description: String,
            user_id: i32,
        }

        let req = serde_json::from_value::<Req>(params.unwrap())?;

        use crate::model::schema::topics::dsl::*;

        let new_topic = topic::NewTopic {
            title: &req.title,
            description: &req.description,
            user_id: req.user_id,
        };
        let result: topic::Topic = diesel::insert_into(topics)
            .values(&new_topic)
            // .returning(id)
            .get_result(&db.conn)?;

        let result = json!({
            "id": result.id
        });

        Ok(Some(result))
    }

    fn get(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        use crate::model::schema::topics::dsl::*;

        let list = {
            if let Some(p) = params {
                let topic_id = p["id"].as_i64().unwrap() as i32;
                topics
                    .filter(id.eq(topic_id))
                    .limit(1)
                    .load::<topic::Topic>(&db.conn)?
            } else {
                topics.load::<topic::Topic>(&db.conn)?
            }
        };

        let result = serde_json::to_value(&list)?;
        Ok(Some(result))
    }

    fn remove(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct Req {
            id: Vec<i32>,
        }

        let req = serde_json::from_value::<Req>(params.unwrap())?;

        use crate::model::schema::topics::dsl::*;

        diesel::delete(topics.filter(id.eq_any(req.id))).execute(&db.conn)?;
        Ok(None)
    }
}

impl Controller for Topic {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        match method {
            "create" => self.create(db, params),
            "get" => self.get(db, params),
            "remove" => self.remove(db, params),
            _ => Err(api::make_error_data(
                api::error::METHOD_NOT_FOUND,
                method.to_string(),
            )),
        }
    }
}

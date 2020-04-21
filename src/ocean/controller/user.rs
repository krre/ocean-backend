use super::Controller;
use crate::db;
use crate::model::user;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use serde_json::json;

pub struct User {}

#[derive(Deserialize)]
struct CreateRequest {
    name: Option<String>,
    password: String,
}

impl User {
    fn create(&self, db: &db::Db, params: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let request: CreateRequest = serde_json::from_value(params.unwrap()).unwrap();

        use crate::model::schema::users::dsl::*;

        let new_user = user::NewUser {
            name: request.name,
            password: &request.password,
        };
        let result: user::User = diesel::insert_into(users)
            .values(&new_user)
            // .returning(id)
            .get_result(&db.conn)
            .unwrap();

        let result = json!({
            "id": result.id
        });

        Some(result)
    }
}

impl Controller for User {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Option<serde_json::Value> {
        match method {
            "create" => self.create(db, params),
            _ => {
                println!("method {} not found", method);
                None
            }
        }
    }
}

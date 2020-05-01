use super::Controller;
use crate::api;
use crate::db;
use crate::json_rpc;
use crate::model::user;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use serde_json::json;
use sha1;

pub struct User {}

#[derive(Deserialize)]
struct CreateRequest {
    name: Option<String>,
    password: String,
}

#[derive(Deserialize)]
struct AuthRequest {
    id: i32,
    password: String,
}

impl User {
    fn create(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        let request: CreateRequest = serde_json::from_value(params.unwrap()).unwrap();

        use crate::model::schema::users::dsl::*;

        let new_user = user::NewUser {
            name: request.name,
            token: "dummy".to_string(),
        };

        let result: user::User = diesel::insert_into(users)
            .values(&new_user)
            // .returning(id)
            .get_result(&db.conn)
            .unwrap();

        let user_id = result.id;
        let user_token = &sha1_token(user_id, request.password);

        diesel::update(users.filter(id.eq(user_id)))
            .set(token.eq(user_token))
            .execute(&db.conn)
            .unwrap();

        let result = json!({
            "id": user_id,
            "token": user_token
        });

        Ok(Some(result))
    }

    fn auth(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        let request: AuthRequest = serde_json::from_value(params.unwrap()).unwrap();

        use crate::model::schema::users::dsl::*;

        let result = users
            .filter(id.eq(request.id))
            .load::<user::User>(&db.conn)
            .unwrap();

        let request_token = sha1_token(request.id, request.password);

        if result.len() == 0 || result[0].token != request_token {
            let error = json_rpc::response::Error {
                code: 42,
                message: "User with id and password not found".to_string(),
                data: None,
            };

            let result = serde_json::to_value(&error).unwrap();
            Ok(Some(result))
        } else {
            let result = json!({ "token": request_token });
            Ok(Some(result))
        }
    }
}

impl Controller for User {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, api::error::Error> {
        match method {
            "create" => self.create(db, params),
            "auth" => self.auth(db, params),
            _ => Err(api::error::Error::new(
                api::error::METHOD_NOT_FOUND,
                Some(method.to_string()),
            )),
        }
    }
}

fn sha1_token(id: i32, password: String) -> String {
    let mut sha = sha1::Sha1::new();
    sha.update((id.to_string() + &password).as_bytes());
    sha.digest().to_string()
}

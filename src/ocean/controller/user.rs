use super::Controller;
use crate::api;
use crate::db;
use crate::model::user;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use serde_json::json;
use sha1;
use std::error::Error;

pub struct User;

impl User {
    fn create(
        &self,
        db: &db::Db,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct Req {
            name: Option<String>,
            password: String,
        }

        let req = serde_json::from_value::<Req>(params.unwrap())?;

        use crate::model::schema::users::dsl::*;

        let new_user = user::NewUser {
            name: req.name,
            token: "dummy".to_string(),
        };

        let result: user::User = diesel::insert_into(users)
            .values(&new_user)
            // .returning(id)
            .get_result(&db.conn)?;

        let user_id = result.id;
        let user_token = &sha1_token(user_id, req.password);

        diesel::update(users.filter(id.eq(user_id)))
            .set(token.eq(user_token))
            .execute(&db.conn)?;

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
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct Req {
            id: i32,
            password: String,
        }

        let req = serde_json::from_value::<Req>(params.unwrap())?;

        use crate::model::schema::users::dsl::*;

        let result = users.filter(id.eq(req.id)).load::<user::User>(&db.conn)?;

        let request_token = sha1_token(req.id, req.password);

        if result.len() == 0 || result[0].token != request_token {
            Err(Box::new(api::Error::new(
                api::error::WRONG_USER_PASSWORD,
                None,
            )))
        } else {
            Ok(Some(json!({ "token": request_token })))
        }
    }
}

impl Controller for User {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        match method {
            "create" => self.create(db, params),
            "auth" => self.auth(db, params),
            _ => Err(Box::new(api::error::Error::new(
                api::error::METHOD_NOT_FOUND,
                Some(method.to_string()),
            ))),
        }
    }
}

fn sha1_token(id: i32, password: String) -> String {
    let mut sha = sha1::Sha1::new();
    sha.update((id.to_string() + &password).as_bytes());
    sha.digest().to_string()
}

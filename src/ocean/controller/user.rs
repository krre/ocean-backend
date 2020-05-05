use super::*;
use crate::api;
use crate::model::user;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

// user.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        name: Option<String>,
        password: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::users::dsl::*;

    let new_user = user::NewUser {
        name: req.name,
        token: "dummy".to_string(),
    };

    let result: user::User = diesel::insert_into(users)
        .values(&new_user)
        // .returning(id)
        .get_result(&data.db.conn)?;

    let user_id = result.id;
    let user_token = &sha1_token(user_id, req.password);

    diesel::update(users.filter(id.eq(user_id)))
        .set(token.eq(user_token))
        .execute(&data.db.conn)?;

    let result = json!({
        "id": user_id,
        "token": user_token
    });

    Ok(Some(result))
}

// user.auth
pub fn auth(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: i32,
        password: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::users::dsl::*;

    let result = users
        .filter(id.eq(req.id))
        .load::<user::User>(&data.db.conn)?;

    let request_token = sha1_token(req.id, req.password);

    if result.is_empty() || result[0].token != request_token {
        Err(api::make_error(api::error::WRONG_USER_PASSWORD))
    } else {
        Ok(Some(json!({ "token": request_token })))
    }
}

fn sha1_token(id: i32, password: String) -> String {
    let mut sha = sha1::Sha1::new();
    sha.update((id.to_string() + &password).as_bytes());
    sha.digest().to_string()
}

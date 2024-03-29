use crate::db;
use crate::types;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref USER_CACHE: Mutex<HashMap<String, types::User>> = Mutex::new(HashMap::new());
}

pub fn init(db: db::Db) {
    use crate::model::schema::user_groups;
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Queryable)]
    struct UserData {
        id: types::Id,
        name: String,
        token: String,
        code: String,
        blocked: bool,
    }

    let list = users
        .inner_join(user_groups)
        .select((
            users::id,
            users::name,
            users::token,
            user_groups::code,
            users::blocked,
        ))
        .load::<UserData>(&db.conn)
        .unwrap();

    for user_data in list {
        let user = types::User {
            id: user_data.id,
            code: user_code(&user_data.code),
            name: user_data.name,
            blocked: user_data.blocked,
        };

        USER_CACHE.lock().unwrap().insert(user_data.token, user);
    }
}

pub fn set(token: &str, user: types::User) {
    USER_CACHE.lock().unwrap().insert(token.to_string(), user);
}

pub fn get(token: &str) -> Option<types::User> {
    USER_CACHE.lock().unwrap().get(token).map(|u| (*u).clone())
}

pub fn update_blocked(id: types::Id, blocked: bool) {
    let mut token: String = String::new();
    let mut user: Option<types::User> = None;

    for (key, value) in USER_CACHE.lock().unwrap().iter() {
        if value.id == id {
            token = (*key).clone();
            user = Some((*value).clone());
            break;
        }
    }

    if let Some(mut u) = user {
        u.blocked = blocked;
        set(&token, u);
    }
}

pub fn user_code(code: &str) -> types::UserCode {
    match code {
        "admin" => types::UserCode::Admin,
        "user" => types::UserCode::User,
        "anonym" => types::UserCode::Anonym,
        _ => panic!("Unknown user code {}", code),
    }
}

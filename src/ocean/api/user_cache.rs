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
        token: String,
        code: String,
    }

    let list = users
        .inner_join(user_groups)
        .select((users::id, users::token, user_groups::code))
        .load::<UserData>(&db.conn)
        .unwrap();

    for user_data in list {
        let user = types::User {
            id: user_data.id,
            code: user_code(&user_data.code),
        };

        USER_CACHE.lock().unwrap().insert(user_data.token, user);
    }
}

pub fn set(token: &String, user: types::User) {
    USER_CACHE.lock().unwrap().insert(token.to_string(), user);
}

pub fn get(token: &String) -> Option<types::User> {
    if let Some(u) = USER_CACHE.lock().unwrap().get(token) {
        Some((*u).clone())
    } else {
        None
    }
}

pub fn user_code(code: &String) -> types::UserCode {
    match code.as_ref() {
        "admin" => types::UserCode::Admin,
        "user" => types::UserCode::User,
        "anonym" => types::UserCode::Anonym,
        _ => panic!(format!("Unknown user code {}", code)),
    }
}

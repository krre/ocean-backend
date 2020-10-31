use super::*;
use crate::api;
use crate::api::user_cache;
use crate::model::user;
use crate::model::user_group;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

// user.getNextId
pub fn get_next_id(data: RequestData) -> RequestResult {
    let id = next_id(&data.db)?;

    let result = json!({
        "id": id,
    });

    Ok(Some(result))
}

fn next_id(db: &db::Db) -> Result<Id, Box<dyn std::error::Error>> {
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;
    let user_id: Id = users
        .select(users::id)
        .order(users::id.desc())
        .first(&db.conn)?;

    Ok(user_id + 1)
}

// user.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        id: Id,
        name: Option<String>,
        code: String,
        token: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;
    let next_id = next_id(&data.db)?;

    if req.id != next_id {
        return Err(api::make_error(api::error::NEXT_ID_EXPIRED));
    }

    let groups = user_groups
        .filter(code.eq(&req.code))
        .first::<user_group::UserGroup>(&data.db.conn)?;

    let new_user = user::NewUser {
        name: req.name,
        group_id: groups.id,
        token: req.token,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(users::id)
        .get_result::<Id>(&data.db.conn)?;

    let user = types::User {
        id: req.id,
        code: user_cache::user_code(&req.code),
    };

    user_cache::set(&new_user.token, user);

    Ok(None)
}

// user.auth
pub fn auth(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups;
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        token: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;
    let user_id;

    if let Some(u) = user_cache::get(&req.token) {
        user_id = u.id;
    } else {
        return Err(api::make_error(api::error::WRONG_USER_PASSWORD));
    }

    let user = users
        .filter(users::id.eq(user_id))
        .first::<user::User>(&data.db.conn)?;

    let user_group = user_groups
        .filter(user_groups::id.eq(user.group_id))
        .first::<user_group::UserGroup>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        code: String,
        name: Option<String>,
    }

    let resp = Resp {
        code: user_group.code,
        name: user.name,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// user.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups;
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Queryable, Serialize)]
    struct User {
        id: types::Id,
        name: Option<String>,
        code: String,
        create_ts: NaiveDateTime,
    }

    let user = users
        .inner_join(user_groups)
        .select((users::id, users::name, user_groups::code, users::create_ts))
        .filter(users::id.eq(data.user.id))
        .first::<User>(&data.db.conn)?;

    let result = serde_json::to_value(&user)?;
    Ok(Some(result))
}

// user.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        name: String,
        code: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let groups = user_groups
        .filter(code.eq(req.code))
        .first::<user_group::UserGroup>(&data.db.conn)?;

    #[derive(AsChangeset)]
    #[table_name = "users"]
    pub struct UpdateUser {
        pub name: String,
        pub group_id: Id,
        pub update_ts: NaiveDateTime,
    }

    let update_user = UpdateUser {
        name: req.name,
        group_id: groups.id,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(users.filter(users::id.eq(data.user.id)))
        .set(&update_user)
        .execute(&data.db.conn)?;

    Ok(None)
}

// user.updateToken
pub fn update_token(data: RequestData) -> RequestResult {
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        token: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    diesel::update(users.filter(id.eq(data.user.id)))
        .set(token.eq(&req.token))
        .execute(&data.db.conn)?;

    let user = types::User {
        id: data.user.id,
        code: data.user.code,
    };

    user_cache::set(&req.token, user);

    Ok(None)
}

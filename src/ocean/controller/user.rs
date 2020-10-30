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

// user.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        name: Option<String>,
        code: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let groups = user_groups
        .filter(code.eq(req.code))
        .limit(1)
        .load::<user_group::UserGroup>(&data.db.conn)?;

    let new_user = user::NewUser {
        name: req.name,
        group_id: groups[0].id,
    };

    let user_id = diesel::insert_into(users)
        .values(&new_user)
        .returning(users::id)
        .get_result::<i32>(&data.db.conn)?;

    let result = json!({
        "id": user_id,
    });

    Ok(Some(result))
}

// user.auth
pub fn auth(data: RequestData) -> RequestResult {
    use crate::model::schema::user_groups;
    use crate::model::schema::user_groups::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        token: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let result = users
        .filter(users::id.eq(req.id))
        .first::<user::User>(&data.db.conn)
        .optional()?;

    if let Some(r) = result {
        if r.token != req.token {
            return Err(api::make_error(api::error::WRONG_USER_PASSWORD));
        }

        let user_group = user_groups
            .filter(user_groups::id.eq(r.group_id))
            .first::<user_group::UserGroup>(&data.db.conn)?;

        Ok(Some(json!({ "code": user_group.code,
             "name": r.name })))
    } else {
        Err(api::make_error(api::error::WRONG_USER_PASSWORD))
    }
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
        id: Id,
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

    diesel::update(users.filter(users::id.eq(req.id)))
        .set(&update_user)
        .execute(&data.db.conn)?;

    Ok(None)
}

// user.changePassword
pub fn change_password(data: RequestData) -> RequestResult {
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        token: String,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    diesel::update(users.filter(id.eq(req.id)))
        .set(token.eq(&req.token))
        .execute(&data.db.conn)?;

    let user = types::User {
        id: data.user.id,
        code: data.user.code,
    };

    user_cache::set(req.token, user);

    Ok(None)
}

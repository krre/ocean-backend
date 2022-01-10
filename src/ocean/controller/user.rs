use super::*;
use crate::api;
use crate::api::user_cache;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Int2, Int4, Int8, Text, Timestamptz};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
struct UserGroup {
    id: Id,
    name: Option<String>,
    code: String,
}

// user.getNextId
pub fn get_next_id(data: RequestData) -> RequestResult {
    let resp = ResponseId {
        id: next_id(&data.db)?,
    };
    let result = serde_json::to_value(&resp)?;
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
        name: String,
        code: String,
        token: String,
    }

    let req: Req = data.params()?;
    let next_id = next_id(&data.db)?;

    if req.id != next_id {
        return Err(api::make_error(api::error::NEXT_ID_EXPIRED));
    }

    let groups = user_groups
        .filter(code.eq(&req.code))
        .first::<UserGroup>(&data.db.conn)?;

    #[derive(Insertable)]
    #[table_name = "users"]
    pub struct NewUser {
        name: String,
        group_id: Id,
        token: String,
        blocked: bool,
    }

    let user_name = req.name.clone();

    let new_user = NewUser {
        name: req.name,
        group_id: groups.id,
        token: req.token,
        blocked: false,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(users::id)
        .get_result::<Id>(&data.db.conn)?;

    let user = types::User {
        id: req.id,
        code: user_cache::user_code(&req.code),
        name: user_name,
        blocked: new_user.blocked,
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

    let req: Req = data.params()?;
    let user_id;

    if let Some(u) = user_cache::get(&req.token) {
        user_id = u.id;
    } else {
        return Err(api::make_error(api::error::WRONG_USER_PASSWORD));
    }

    #[derive(Queryable, Serialize)]
    pub struct User {
        id: Id,
        name: String,
        token: String,
        group_id: Id,
        create_ts: NaiveDateTime,
        update_ts: NaiveDateTime,
        gender: i16,
        blocked: bool,
    }

    let user = users
        .filter(users::id.eq(user_id))
        .first::<User>(&data.db.conn)?;

    let user_group = user_groups
        .filter(user_groups::id.eq(user.group_id))
        .first::<UserGroup>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        code: String,
        name: String,
        gender: i16,
    }

    let resp = Resp {
        code: user_group.code,
        name: user.name,
        gender: user.gender,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// user.logout
pub fn logout(_data: RequestData) -> RequestResult {
    Ok(None)
}

// user.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    let req: RequestId = data.params()?;

    #[derive(QueryableByName, Serialize)]
    struct User {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Text"]
        name: String,
        #[sql_type = "Text"]
        code: String,
        #[sql_type = "Int2"]
        gender: i16,
        #[sql_type = "Bool"]
        blocked: bool,
        #[sql_type = "Timestamptz"]
        create_ts: NaiveDateTime,
        #[sql_type = "Int8"]
        mandela_count: i64,
        #[sql_type = "Int8"]
        comment_count: i64,
        #[sql_type = "Int8"]
        forum_topic_count: i64,
        #[sql_type = "Int8"]
        forum_post_count: i64,
        #[sql_type = "Int8"]
        like_count: i64,
        #[sql_type = "Int8"]
        dislike_count: i64,
    }

    use diesel::dsl::*;

    let user = sql_query(format!(
        "SELECT u.id, u.name, ug.code, u.gender, u.blocked, u.create_ts,
            (SELECT count(*) FROM mandels WHERE user_id = u.id) AS mandela_count,
            (SELECT count(*) FROM comments WHERE user_id = u.id) AS comment_count,
            (SELECT count(*) FROM forum_topics WHERE user_id = u.id) AS forum_topic_count,
            (SELECT count(*) FROM forum_posts WHERE user_id = u.id) AS forum_post_count,
            (SELECT count(c.*)
            FROM comments AS c
                JOIN likes AS l ON l.comment_id = c.id
            WHERE c.user_id = $1 and l.value = 0) +
            (SELECT count(fp.*)
            FROM forum_posts AS fp
                JOIN likes AS l ON l.post_id = fp.id
            WHERE fp.user_id = $1 and l.value = 0) AS like_count,
            (SELECT count(c.*)
            FROM comments AS c
                JOIN likes AS l ON l.comment_id = c.id
            WHERE c.user_id = $1 and l.value = 1) +
            (SELECT count(fp.*)
            FROM forum_posts AS fp
                JOIN likes AS l ON l.post_id = fp.id
            WHERE fp.user_id = $1 and l.value = 1) AS dislike_count
        FROM users AS u
            JOIN user_groups AS ug ON ug.id = u.group_id
        WHERE u.id = $1"
    ))
    .bind::<Int4, _>(req.id)
    .load::<User>(&data.db.conn)?;

    if !user.is_empty() {
        let result = serde_json::to_value(&user[0])?;
        Ok(Some(result))
    } else {
        Err(api::make_error(api::error::RECORD_NOT_FOUND))
    }
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
        gender: i16,
        blocked: bool,
    }

    let req: Req = data.params()?;

    let groups = user_groups
        .filter(code.eq(req.code))
        .first::<UserGroup>(&data.db.conn)?;

    #[derive(AsChangeset)]
    #[table_name = "users"]
    pub struct UpdateUser {
        pub name: String,
        pub gender: i16,
        pub group_id: Id,
        pub blocked: bool,
        pub update_ts: NaiveDateTime,
    }

    let update_user = UpdateUser {
        name: req.name,
        gender: req.gender,
        group_id: groups.id,
        blocked: req.blocked,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(users.filter(users::id.eq(req.id)))
        .set(&update_user)
        .execute(&data.db.conn)?;

    user_cache::update_blocked(req.id, req.blocked);

    Ok(None)
}

// user.updateProfile
pub fn update_profile(data: RequestData) -> RequestResult {
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        name: String,
        gender: i16,
    }

    let req: Req = data.params()?;

    #[derive(AsChangeset)]
    #[table_name = "users"]
    pub struct UpdateUser {
        pub name: String,
        pub gender: i16,
        pub update_ts: NaiveDateTime,
    }

    let update_user = UpdateUser {
        name: req.name,
        gender: req.gender,
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

    let req: Req = data.params()?;

    diesel::update(users.filter(id.eq(data.user.id)))
        .set(token.eq(&req.token))
        .execute(&data.db.conn)?;

    let user = types::User {
        id: data.user.id,
        code: data.user.code,
        name: data.user.name,
        blocked: data.user.blocked,
    };

    user_cache::set(&req.token, user);

    Ok(None)
}

// user.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::users::dsl::*;
    let req: RequestId = data.params()?;

    diesel::delete(users.filter(id.eq(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

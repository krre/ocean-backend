use self::mandela;
use super::*;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// comment.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        mandela_id: Id,
        message: String,
    }

    let req: Req = data.params()?;

    use crate::model::schema::comments;

    #[derive(Insertable)]
    #[table_name = "comments"]
    pub struct NewComment {
        mandela_id: Id,
        user_id: Id,
        message: String,
    }

    let new_comment = NewComment {
        mandela_id: req.mandela_id,
        user_id: data.user.id,
        message: req.message,
    };

    use crate::model::schema::comments::dsl::*;
    use crate::model::schema::mandels;
    use crate::model::schema::users;

    diesel::insert_into(comments)
        .values(&new_comment)
        .execute(&data.db.conn)?;

    let mandela_title = mandels::table
        .select((
            mandels::id,
            mandels::title_mode,
            mandels::title,
            mandels::what,
            mandels::before,
            mandels::after,
        ))
        .filter(mandels::id.eq(new_comment.mandela_id))
        .first::<mandela::MandelaTitle>(&data.db.conn)?;

    let user_name = users::table
        .select(users::name.nullable())
        .filter(users::id.eq(data.user.id))
        .first::<Option<String>>(&data.db.conn)?;

    let comment_message = format!(
        "Каталог
{}
{}

{}",
        format_mandela_title(mandela_title),
        user_name.unwrap(),
        new_comment.message
    );

    telegram_bot::send_admin_message(comment_message);

    Ok(None)
}

// comment.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::comments;
    use crate::model::schema::comments::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        mandela_id: Id,
        offset: i64,
        limit: i64,
    }

    let req: Req = data.params()?;

    #[derive(Queryable, Serialize)]
    pub struct Comment {
        pub id: Id,
        pub user_id: Id,
        pub user_name: String,
        pub message: String,
        pub create_ts: NaiveDateTime,
        pub update_ts: NaiveDateTime,
    }

    let list = comments
        .inner_join(users)
        .select((
            comments::id,
            users::id,
            users::name,
            message,
            comments::create_ts,
            comments::update_ts,
        ))
        .filter(mandela_id.eq(req.mandela_id))
        .order(comments::id.asc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<Comment>(&data.db.conn)?;

    let total_count: i64 = comments
        .filter(mandela_id.eq(req.mandela_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        comments: Vec<Comment>,
    };

    let resp = Resp {
        total_count: total_count,
        comments: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// comment.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::comments;
    use crate::model::schema::comments::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        message: String,
    }

    let req: Req = data.params()?;

    #[derive(AsChangeset)]
    #[table_name = "comments"]
    pub struct UpdateComment {
        pub message: String,
        pub update_ts: NaiveDateTime,
    }

    let update_comment = UpdateComment {
        message: req.message,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(comments.filter(id.eq(req.id)))
        .set(&update_comment)
        .execute(&data.db.conn)?;

    Ok(None)
}

// comment.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::comments::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req: Req = data.params()?;

    diesel::delete(comments.filter(id.eq(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

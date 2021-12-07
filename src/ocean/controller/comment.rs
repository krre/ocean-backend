use self::mandela;
use super::*;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Int2, Int4, Int8, Nullable, Text, Timestamptz};
use serde::{Deserialize, Serialize};

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
    use crate::model::schema::comments::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        mandela_id: Id,
        offset: i32,
        limit: i32,
    }

    let req: Req = data.params()?;

    #[derive(QueryableByName, Serialize)]
    pub struct Comment {
        #[sql_type = "Int4"]
        pub id: Id,
        #[sql_type = "Int4"]
        pub user_id: Id,
        #[sql_type = "Text"]
        pub user_name: String,
        #[sql_type = "Text"]
        pub message: String,
        #[sql_type = "Int8"]
        pub like_count: i64,
        #[sql_type = "Int8"]
        pub dislike_count: i64,
        #[sql_type = "Nullable<Int2>"]
        pub like: Option<i16>,
        #[sql_type = "Timestamptz"]
        pub create_ts: NaiveDateTime,
        #[sql_type = "Timestamptz"]
        pub update_ts: NaiveDateTime,
    }

    let list = diesel::dsl::sql_query(
        "SELECT c.id, u.id AS user_id, u.name AS user_name, message, l.value AS like, c.create_ts, c.update_ts,
            (SELECT count(*) FROM likes WHERE comment_id = c.id AND value = 0) AS like_count,
            (SELECT count(*) FROM likes WHERE comment_id = c.id AND value = 1) AS dislike_count
        FROM comments AS c
            JOIN users AS u ON u.id = c.user_id
            LEFT JOIN likes AS l ON l.comment_id = c.id AND l.user_id = $1
        WHERE mandela_id = $2
        ORDER BY c.id ASC
        OFFSET $3
        LIMIT $4",
    )
    .bind::<Int4, _>(data.user.id)
    .bind::<Int4, _>(req.mandela_id)
    .bind::<Int4, _>(req.offset)
    .bind::<Int4, _>(req.limit)
    .load::<Comment>(&data.db.conn)?;

    let total_count: i64 = comments
        .filter(mandela_id.eq(req.mandela_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        comments: Vec<Comment>,
    }

    let resp = Resp {
        total_count,
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
    let req: RequestId = data.params()?;

    diesel::delete(comments.filter(id.eq(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

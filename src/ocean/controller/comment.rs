use super::*;
use crate::model::comment;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// comment.create
pub fn create(data: RequestData) -> RequestResult {
    let new_comment = serde_json::from_value::<comment::NewComment>(data.params.unwrap())?;

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
        .first::<model::mandela::MandelaTitle>(&data.db.conn)?;

    let user_name = users::table
        .select(users::name.nullable())
        .filter(users::id.eq(new_comment.user_id))
        .first::<Option<String>>(&data.db.conn)?;

    let final_user_name: String = if let Some(n) = user_name {
        n
    } else {
        if new_comment.user_id == 2 {
            "Лютый конспиролог".into()
        } else {
            "Конспиролог".into()
        }
    };

    let comment_message = format!(
        "{}
{}

{}",
        format_mandela_title(mandela_title),
        final_user_name,
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

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

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
        .load::<comment::Comment>(&data.db.conn)?;

    let total_count: i64 = comments
        .filter(mandela_id.eq(req.mandela_id))
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        comments: Vec<comment::Comment>,
    };

    let resp = serde_json::to_value(&Resp {
        total_count: total_count,
        comments: list,
    })?;

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

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

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
    let comment_id = data.params.unwrap()["id"].as_i64().unwrap() as i32;

    diesel::delete(comments.filter(id.eq(comment_id))).execute(&data.db.conn)?;
    Ok(None)
}

use crate::controller::*;
use crate::types::Id;
use diesel::prelude::*;
use serde::Deserialize;

// like.create
pub fn create(mut data: RequestData) -> RequestResult {
    use crate::model::schema::likes;
    use crate::model::schema::likes::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        comment_id: Option<Id>,
        post_id: Option<Id>,
        action: i16,
    }

    let req: Req = data.params()?;

    #[derive(Insertable, Deserialize)]
    #[diesel(table_name = likes)]
    struct NewLike {
        user_id: Id,
        comment_id: Option<Id>,
        post_id: Option<Id>,
        value: i16,
    }

    let new_like = NewLike {
        user_id: data.user.id,
        comment_id: req.comment_id,
        post_id: req.post_id,
        value: req.action,
    };

    diesel::insert_into(likes)
        .values(&new_like)
        .execute(&mut data.db.conn)?;

    Ok(None)
}

// like.delete
pub fn delete(mut data: RequestData) -> RequestResult {
    use crate::model::schema::likes::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        comment_id: Option<Id>,
        post_id: Option<Id>,
    }

    let req: Req = data.params()?;

    if let Some(like_comment_id) = req.comment_id {
        diesel::delete(likes.filter(comment_id.eq(like_comment_id).and(user_id.eq(data.user.id))))
            .execute(&mut data.db.conn)?;
    }

    if let Some(like_post_id) = req.post_id {
        diesel::delete(likes.filter(post_id.eq(like_post_id).and(user_id.eq(data.user.id))))
            .execute(&mut data.db.conn)?;
    }

    Ok(None)
}

// like.getUsers
pub fn get_users(mut data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        comment_id: Option<Id>,
        post_id: Option<Id>,
    }

    let req: Req = data.params()?;

    #[derive(Queryable, Serialize)]
    struct LikeUser {
        id: Id,
        name: String,
        action: i16,
    }

    use crate::model::schema::likes::dsl::*;
    use crate::model::schema::users;

    let mut query = likes
        .inner_join(users::table)
        .select((users::id, users::name, value))
        .into_boxed();

    if let Some(filter_comment_id) = req.comment_id {
        query = query.filter(comment_id.eq(filter_comment_id));
    }

    if let Some(filter_post_id) = req.post_id {
        query = query.filter(post_id.eq(filter_post_id));
    }

    let like_users = query
        .order((value.asc(), users::name.asc()))
        .load::<LikeUser>(&mut data.db.conn)?;

    let result = serde_json::to_value(&like_users)?;
    Ok(Some(result))
}

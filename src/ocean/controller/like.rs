use crate::controller::*;
use crate::types::Id;
use diesel::prelude::*;
use serde::Deserialize;

// like.create
pub fn create(data: RequestData) -> RequestResult {
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
    #[table_name = "likes"]
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
        .execute(&data.db.conn)?;

    Ok(None)
}

// like.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::likes::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        comment_id: Option<Id>,
        post_id: Option<Id>,
    }

    let req: Req = data.params()?;

    let query = diesel::delete(
        likes.filter(
            comment_id
                .eq(req.comment_id)
                .and(post_id.eq(req.post_id).and(user_id.eq(data.user.id))),
        ),
    );

    let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    println!("Debug query: {:?}", debug);

    diesel::delete(
        likes.filter(
            comment_id
                .eq(req.comment_id)
                .and(post_id.eq(req.post_id).and(user_id.eq(data.user.id))),
        ),
    )
    .execute(&data.db.conn)?;
    Ok(None)
}

use super::*;
use crate::model::comment;
use diesel::prelude::*;

// comment.create
pub fn create(data: RequestData) -> RequestResult {
    let new_comment = serde_json::from_value::<comment::NewComment>(data.params.unwrap())?;

    use crate::model::schema::comments::dsl::*;

    diesel::insert_into(comments)
        .values(&new_comment)
        .execute(&data.db.conn)?;

    Ok(None)
}

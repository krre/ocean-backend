use crate::controller::*;
use diesel::prelude::*;
use serde::Deserialize;

// forumCategory.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Insertable, Deserialize)]
    #[table_name = "forum_categories"]
    struct NewForumCategory {
        name: String,
        order_index: i16,
    }

    let new_forum_category = serde_json::from_value::<NewForumCategory>(data.params.unwrap())?;

    diesel::insert_into(forum_categories)
        .values(&new_forum_category)
        .execute(&data.db.conn)?;

    Ok(None)
}

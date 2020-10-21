use crate::controller::*;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

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

// forumCategory.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    pub struct ForumCategory {
        name: String,
        order_index: i16,
    }

    let forum_category = forum_categories
        .select((name, order_index))
        .filter(id.eq(req.id))
        .first::<ForumCategory>(&data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_category)?;
    Ok(Some(result))
}

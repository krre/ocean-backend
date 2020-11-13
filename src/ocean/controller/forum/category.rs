use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// forum.category.create
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

// forum.category.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
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

// forum.category.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        name: String,
        order_index: i16,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(AsChangeset)]
    #[table_name = "forum_categories"]
    pub struct UpdateForumCategory {
        pub name: String,
        pub order_index: i16,
        pub update_ts: NaiveDateTime,
    }

    let update_forum_category = UpdateForumCategory {
        name: req.name,
        order_index: req.order_index,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(forum_categories.filter(id.eq(req.id)))
        .set(&update_forum_category)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.category.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories::dsl::*;
    let forum_category_id = data.params.unwrap()["id"].as_i64().unwrap() as i32;

    diesel::delete(forum_categories.filter(id.eq(forum_category_id))).execute(&data.db.conn)?;
    Ok(None)
}

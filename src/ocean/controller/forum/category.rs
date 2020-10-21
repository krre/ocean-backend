use crate::controller::*;
use chrono::prelude::*;
use chrono::NaiveDateTime;
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

// forumCategory.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: i32,
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
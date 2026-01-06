use crate::controller::*;
use crate::types::Id;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// forum.category.create
pub fn create(mut data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Insertable, Deserialize)]
    #[table_name = "forum_categories"]
    struct Req {
        name: String,
        order_index: i16,
    }

    let req: Req = data.params()?;

    diesel::insert_into(forum_categories)
        .values(&req)
        .execute(&mut data.db.conn)?;

    Ok(None)
}

// forum.category.getOne
pub fn get_one(mut data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories::dsl::*;

    let req: RequestId = data.params()?;

    #[derive(Queryable, Serialize)]
    pub struct ForumCategory {
        name: String,
        order_index: i16,
    }

    let forum_category = forum_categories
        .select((name, order_index))
        .filter(id.eq(req.id))
        .first::<ForumCategory>(&mut data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_category)?;
    Ok(Some(result))
}

// forum.category.update
pub fn update(mut data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        name: String,
        order_index: i16,
    }

    let req: Req = data.params()?;

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
        .execute(&mut data.db.conn)?;

    Ok(None)
}

// forum.category.delete
pub fn delete(mut data: RequestData) -> RequestResult {
    let req: RequestId = data.params()?;

    use crate::model::schema::forum_categories::dsl::*;
    diesel::delete(forum_categories.filter(id.eq(req.id))).execute(&mut data.db.conn)?;
    Ok(None)
}

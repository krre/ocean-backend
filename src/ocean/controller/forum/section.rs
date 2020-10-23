use crate::controller::*;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// forumCategory.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Insertable, Deserialize)]
    #[table_name = "forum_sections"]
    struct NewForumSection {
        category_id: i32,
        name: String,
        order_index: i16,
    }

    let new_forum_section = serde_json::from_value::<NewForumSection>(data.params.unwrap())?;

    diesel::insert_into(forum_sections)
        .values(&new_forum_section)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forumCategory.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    pub struct ForumSection {
        category_id: i32,
        name: String,
        order_index: i16,
    }

    let forum_section = forum_sections
        .select((category_id, name, order_index))
        .filter(id.eq(req.id))
        .first::<ForumSection>(&data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_section)?;
    Ok(Some(result))
}

// forumCategory.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: i32,
        name: String,
        order_index: i16,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(AsChangeset)]
    #[table_name = "forum_sections"]
    pub struct UpdateForumSection {
        pub name: String,
        pub order_index: i16,
        pub update_ts: NaiveDateTime,
    }

    let update_forum_section = UpdateForumSection {
        name: req.name,
        order_index: req.order_index,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(forum_sections.filter(id.eq(req.id)))
        .set(&update_forum_section)
        .execute(&data.db.conn)?;

    Ok(None)
}

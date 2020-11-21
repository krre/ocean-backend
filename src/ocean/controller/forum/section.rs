use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// forum.section.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Insertable, Deserialize)]
    #[table_name = "forum_sections"]
    struct NewForumSection {
        category_id: Id,
        name: String,
        order_index: i16,
    }

    let new_forum_section = serde_json::from_value::<NewForumSection>(data.params.unwrap())?;

    diesel::insert_into(forum_sections)
        .values(&new_forum_section)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.section.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    pub struct ForumSection {
        category_id: Id,
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

// forum.section.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
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

// forum.section.delete
pub fn delete(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections::dsl::*;
    let forum_section_id = data.params.unwrap()["id"].as_i64().unwrap() as i32;

    diesel::delete(forum_sections.filter(id.eq(forum_section_id))).execute(&data.db.conn)?;
    Ok(None)
}
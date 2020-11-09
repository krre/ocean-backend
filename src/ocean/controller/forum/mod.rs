use crate::controller::*;
use crate::types::Id;
use diesel::prelude::*;
use serde::Serialize;

pub mod category;
pub mod section;

// forum.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories;
    use crate::model::schema::forum_sections;

    #[derive(Queryable, Serialize)]
    pub struct Category {
        id: Id,
        name: String,
    }

    let categories = forum_categories::table
        .select((forum_categories::id, forum_categories::name))
        .order(forum_categories::order_index.asc())
        .load::<Category>(&data.db.conn)?;

    #[derive(Queryable, Serialize)]
    pub struct Section {
        id: Id,
        name: String,
        category_id: Id,
    }

    let sections = forum_sections::table
        .select((
            forum_sections::id,
            forum_sections::name,
            forum_sections::category_id,
        ))
        .order(forum_sections::order_index.asc())
        .load::<Section>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        categories: Vec<Category>,
        sections: Vec<Section>,
    };

    let resp = serde_json::to_value(&Resp {
        categories: categories,
        sections: sections,
    })?;

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

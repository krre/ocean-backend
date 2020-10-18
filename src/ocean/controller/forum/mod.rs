use crate::controller::*;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub mod category;

// forum.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_categories::dsl::*;

    #[derive(Queryable, Serialize)]
    pub struct Category {
        id: i32,
        name: String,
    }

    let list = forum_categories
        .select((id, name))
        .order(order_index.asc())
        .load::<Category>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        categories: Vec<Category>,
    };

    let resp = serde_json::to_value(&Resp { categories: list })?;

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

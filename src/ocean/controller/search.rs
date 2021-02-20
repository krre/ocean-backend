use super::*;
use crate::types::Id;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// search.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        text: String,
        #[serde(rename(deserialize = "type"))]
        type_: i8,
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    if req.text.is_empty() {
        let data = r#"[]"#;
        let result = serde_json::from_str(data)?;
        return Ok(Some(result));
    }

    let text = format!("%{}%", req.text).to_string();

    #[derive(Queryable, Serialize)]
    struct Mandela {
        id: Id,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
    }

    use crate::model::schema::mandels::dsl::*;

    let list = mandels
        .select((id, title_mode, title, what, before, after))
        .filter(
            title
                .ilike(&text)
                .or(what.ilike(&text))
                .or(before.ilike(&text))
                .or(after.ilike(&text))
                .or(description.ilike(&text)),
        )
        .limit(req.limit)
        .offset(req.offset)
        .load::<Mandela>(&data.db.conn)?;

    let result = serde_json::to_value(&list)?;

    Ok(Some(result))
}

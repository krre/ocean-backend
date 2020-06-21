use super::*;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// search.getById
pub fn get_by_id(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Option<i32>,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let search_id;

    if let Some(i) = req.id {
        if i <= 0 {
            return Ok(None);
        } else {
            search_id = i;
        }
    } else {
        return Ok(None);
    }

    #[derive(Queryable, Serialize)]
    struct Mandela {
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
    }

    use crate::model::schema::mandels::dsl::*;

    let mandela = mandels
        .select((title_mode, title, what, before, after))
        .filter(id.eq(search_id))
        .first::<Mandela>(&data.db.conn)
        .optional();

    if let Ok(md) = mandela {
        let result = serde_json::to_value(&md)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

// search.getByContent
pub fn get_by_content(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        content: String,
        search_title: bool,
        search_description: bool,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    if req.content.is_empty() || !(req.search_title || req.search_description) {
        let data = r#"[]"#;
        let result = serde_json::from_str(data)?;
        return Ok(Some(result));
    }

    let content = format!("%{}%", req.content).to_string();

    #[derive(Queryable, Serialize)]
    struct Mandela {
        id: i32,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
    }

    use crate::model::schema::mandels::dsl::*;

    let mut query = mandels
        .select((id, title_mode, title, what, before, after))
        .into_boxed();

    if req.search_title {
        query = query.filter(
            title
                .ilike(&content)
                .or(what.ilike(&content))
                .or(before.ilike(&content))
                .or(after.ilike(&content)),
        );
    }

    if req.search_description {
        query = query.or_filter(description.ilike(&content));
    }

    let search_mandels = query.limit(50).load::<Mandela>(&data.db.conn)?;
    let result = serde_json::to_value(&search_mandels)?;

    Ok(Some(result))
}

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

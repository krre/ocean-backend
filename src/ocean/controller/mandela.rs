use super::*;
use crate::model::date_serializer;
use crate::model::mandela;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

// mandela.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        title: String,
        description: String,
        images: serde_json::Value,
        videos: serde_json::Value,
        links: serde_json::Value,
        user_id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::mandels::dsl::*;

    let new_mandela = mandela::NewMandela {
        title: req.title,
        description: req.description,
        images: req.images,
        videos: req.videos,
        links: req.links,
        user_id: req.user_id,
    };
    let mandela_id = diesel::insert_into(mandels)
        .values(&new_mandela)
        .returning(id)
        .get_result::<i32>(&data.db.conn)?;

    let result = json!({ "id": mandela_id });

    Ok(Some(result))
}

// mandela.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        id: i32,
        title: String,
        description: String,
        images: serde_json::Value,
        videos: serde_json::Value,
        links: serde_json::Value,
        user_id: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    let update_mandela = mandela::UpdateMandela {
        title: req.title,
        description: req.description,
        images: req.images,
        videos: req.videos,
        links: req.links,
        user_id: req.user_id,
    };

    diesel::update(mandels.filter(mandels::id.eq(req.id)))
        .set(&update_mandela)
        .execute(&data.db.conn)?;

    Ok(None)
}

// mandela.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::mandels::dsl::*;

    let mandela_id = data.params.unwrap()["id"].as_i64().unwrap() as i32;
    let record = mandels
        .filter(id.eq(mandela_id))
        .limit(1)
        .load::<mandela::Mandela>(&data.db.conn)?;

    let result = serde_json::to_value(&record)?;
    Ok(Some(result))
}

// mandela.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        offset: i64,
        limit: i64,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Queryable, Serialize)]
    struct MandelaResp {
        id: i32,
        title: String,
        #[serde(with = "date_serializer")]
        create_ts: NaiveDateTime,
        name: Option<String>,
        user_id: i32,
    }

    let list = mandels
        .inner_join(users)
        .select((
            mandels::id,
            title,
            mandels::create_ts,
            users::name,
            users::id,
        ))
        .order(mandels::id.desc())
        .offset(req.offset)
        .limit(req.limit)
        .load::<MandelaResp>(&data.db.conn)?;

    let total_count: i64 = mandels
        .select(diesel::dsl::count_star())
        .first(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        mandels: Vec<MandelaResp>,
    };

    let resp = serde_json::to_value(&Resp {
        total_count: total_count,
        mandels: list,
    })?;

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// mandela.delete
pub fn delete(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Vec<i32>,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use crate::model::schema::mandels::dsl::*;

    diesel::delete(mandels.filter(id.eq_any(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

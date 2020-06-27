use super::*;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// rating.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        vote: i16,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    println!("{}", req.vote);

    #[derive(Queryable, Serialize)]
    struct Mandela {
        id: i32,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        // count: i64,
    }

    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    use crate::model::schema::votes;
    use crate::model::schema::votes::dsl::*;
    use diesel::dsl::*;

    let list = mandels
        .left_join(votes.on(votes::mandela_id.eq(mandels::id)))
        .select((
            mandels::id,
            title_mode,
            title,
            what,
            before,
            after,
            // count(vote),
        ))
        .filter(vote.eq(req.vote))
        .order_by(count(vote).desc())
        .group_by(mandels::id)
        .limit(50)
        .load::<Mandela>(&data.db.conn)?;

    let result = serde_json::to_value(&list)?;
    Ok(Some(result))
}

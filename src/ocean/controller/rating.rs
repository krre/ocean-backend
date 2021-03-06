use super::*;
use crate::types::Id;
use serde::{Deserialize, Serialize};

// rating.getMandels
pub fn get_mandels(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        vote: i16,
        limit: i32,
        offset: i32,
    }

    let req: Req = data.params()?;

    use diesel::prelude::*;
    use diesel::sql_types::Int2;
    use diesel::sql_types::Int4;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Text;

    #[derive(QueryableByName, Serialize)]
    struct Mandela {
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int4"]
        title_mode: i32,
        #[sql_type = "Text"]
        title: String,
        #[sql_type = "Text"]
        what: String,
        #[sql_type = "Text"]
        before: String,
        #[sql_type = "Text"]
        after: String,
        #[sql_type = "Int8"]
        count: i64,
    }

    #[derive(QueryableByName)]
    struct TotalCount {
        #[sql_type = "Int8"]
        count: i64,
    }

    use diesel::dsl::*;

    let list = sql_query(
        "SELECT m.id, title_mode, title, what, before, after, count(*)
        FROM mandels AS m
        LEFT JOIN votes AS v on v.mandela_id = m.id
        WHERE vote = $1
        GROUP BY m.id
        ORDER BY count DESC, m.id ASC
        LIMIT $2
        OFFSET $3",
    )
    .bind::<Int2, _>(req.vote)
    .bind::<Int4, _>(req.limit)
    .bind::<Int4, _>(req.offset)
    .load::<Mandela>(&data.db.conn)?;

    let total_count = sql_query(
        "SELECT COUNT(DISTINCT mandela_id) from votes
        WHERE vote = $1",
    )
    .bind::<Int2, _>(req.vote)
    .load::<TotalCount>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        mandels: Vec<Mandela>,
    }

    let resp = Resp {
        total_count: total_count[0].count,
        mandels: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// rating.getUsers
pub fn get_users(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        limit: i32,
        offset: i32,
    }

    let req: Req = data.params()?;

    use diesel::prelude::*;
    use diesel::sql_types::Int4;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Text;

    #[derive(QueryableByName, Serialize)]
    struct User {
        #[sql_type = "Text"]
        name: String,
        #[sql_type = "Int4"]
        id: Id,
        #[sql_type = "Int8"]
        count: i64,
    }

    let list = diesel::dsl::sql_query(
        "SELECT u.name, u.id, count(m.*)
        FROM users AS u
        INNER JOIN mandels as m on m.user_id = u.id
        GROUP BY u.id
        ORDER BY count DESC, u.id ASC
        LIMIT $1
        OFFSET $2",
    )
    .bind::<Int4, _>(req.limit)
    .bind::<Int4, _>(req.offset)
    .load::<User>(&data.db.conn)?;

    #[derive(QueryableByName)]
    struct UserCount {
        #[sql_type = "Int8"]
        count: i64,
    }

    let user_count = diesel::dsl::sql_query(
        "SELECT count(*) FROM
            (SELECT count(u.id) FROM users AS u
            INNER JOIN mandels AS m ON m.user_id = u.id
            GROUP BY u.id) AS user_count",
    )
    .load::<UserCount>(&data.db.conn)?;

    #[derive(Serialize)]
    struct Resp {
        user_count: i64,
        users: Vec<User>,
    }

    let resp = Resp {
        user_count: user_count[0].count,
        users: list,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

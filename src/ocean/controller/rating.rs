use super::*;
use serde::Deserialize;
use serde::Serialize;

// rating.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        vote: i16,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    use diesel::prelude::*;
    use diesel::sql_types::Int2;
    use diesel::sql_types::Int4;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Text;

    #[derive(QueryableByName, Serialize)]
    struct Mandela {
        #[sql_type = "Int4"]
        id: i32,
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

    use diesel::dsl::*;

    let list = sql_query(
        "SELECT m.id, title_mode, title, what, before, after, count(*)
        FROM mandels AS m
        LEFT JOIN votes AS v on v.mandela_id = m.id
        WHERE vote = $1
        GROUP BY m.id
        ORDER BY count DESC
        LIMIT 50",
    )
    .bind::<Int2, _>(req.vote)
    .load::<Mandela>(&data.db.conn)?;

    let result = serde_json::to_value(&list)?;
    Ok(Some(result))
}

// rating.getUsers
pub fn get_users(data: RequestData) -> RequestResult {
    use diesel::prelude::*;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Text;

    #[derive(QueryableByName, Serialize)]
    struct User {
        #[sql_type = "Text"]
        name: String,
        #[sql_type = "Int8"]
        count: i64,
    }

    use diesel::dsl::*;

    let list = sql_query(
        "SELECT u.name, count(m.*)
        FROM users AS u
        INNER JOIN mandels as m on m.user_id = u.id
        GROUP BY u.name
        ORDER BY count DESC
        LIMIT 50",
    )
    .load::<User>(&data.db.conn)?;

    let result = serde_json::to_value(&list)?;
    Ok(Some(result))
}

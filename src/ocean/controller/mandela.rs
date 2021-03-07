use super::*;
use crate::api;
use crate::telegram_bot;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Int2;
use diesel::sql_types::Int4;
use diesel::sql_types::Int8;
use diesel::sql_types::Text;
use diesel::sql_types::Timestamptz;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Queryable)]
pub struct MandelaTitle {
    pub id: Id,
    pub title_mode: i32,
    pub title: String,
    pub what: String,
    pub before: String,
    pub after: String,
}

#[derive(QueryableByName, Serialize)]
struct Votes {
    #[sql_type = "Int2"]
    vote: i16,
    #[sql_type = "Int8"]
    count: i64,
}

#[derive(QueryableByName, Serialize)]
pub struct Comment {
    #[sql_type = "Int4"]
    id: Id,
    #[sql_type = "Int4"]
    mandela_id: Id,
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
    #[sql_type = "Text"]
    message: String,
    #[sql_type = "Int4"]
    user_id: i32,
    #[sql_type = "Text"]
    user_name: String,
    #[sql_type = "Timestamptz"]
    create_ts: NaiveDateTime,
    #[sql_type = "Int8"]
    comment_count: i64,
}

fn update_categories(
    conn: &PgConnection,
    mandela_id: Id,
    category_numbers: Vec<i16>,
) -> RequestResult {
    use crate::model::schema::categories;

    #[derive(Queryable, Serialize, Debug)]
    pub struct CategoryNumber {
        id: Id,
        number: i16,
    }

    let current_numbers = categories::table
        .select((categories::id, categories::number))
        .filter(categories::mandela_id.eq(mandela_id))
        .load::<CategoryNumber>(conn)?;

    let mut insert_numbers = category_numbers.clone();

    for category in current_numbers.iter() {
        if !category_numbers.contains(&category.number) {
            diesel::delete(categories::table.filter(categories::id.eq(category.id)))
                .execute(conn)?;
        } else {
            let index = insert_numbers
                .iter()
                .position(|&r| r == category.number)
                .unwrap();
            insert_numbers.remove(index);
        }
    }

    #[derive(Insertable, Deserialize)]
    #[table_name = "categories"]
    pub struct NewCategoryNumber {
        mandela_id: Id,
        number: i16,
    }

    for number in insert_numbers.into_iter() {
        let new_category_number = NewCategoryNumber {
            mandela_id: mandela_id,
            number: number,
        };

        diesel::insert_into(categories::table)
            .values(&new_category_number)
            .execute(conn)?;
    }

    Ok(None)
}

// mandela.create
pub fn create(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        description: String,
        categories: serde_json::Value,
    }

    let req: Req = data.params()?;

    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;

    #[derive(Insertable)]
    #[table_name = "mandels"]
    struct NewMandela {
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        description: String,
        user_id: Id,
    }

    let new_mandela = NewMandela {
        title_mode: req.title_mode,
        title: req.title,
        what: req.what,
        before: req.before,
        after: req.after,
        description: req.description,
        user_id: data.user.id,
    };

    let mandela_id = diesel::insert_into(mandels)
        .values(&new_mandela)
        .returning(id)
        .get_result::<Id>(&data.db.conn)?;

    let category_numbers: Vec<i16> = serde_json::from_value(req.categories).unwrap();
    update_categories(&data.db.conn, mandela_id, category_numbers)?;

    let mut message = format_mandela_title(mandela::MandelaTitle {
        id: mandela_id,
        title: new_mandela.title,
        title_mode: new_mandela.title_mode,
        what: new_mandela.what,
        before: new_mandela.before,
        after: new_mandela.after,
    });

    use crate::model::schema::users;
    let user_name: String = users::table
        .select(users::name)
        .filter(users::id.eq(data.user.id))
        .first(&data.db.conn)?;

    message = message + "\n" + &user_name;

    telegram_bot::send_message(message);

    let result = json!({ "id": mandela_id });
    Ok(Some(result))
}

// mandela.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    #[derive(Deserialize)]
    struct Req {
        id: Id,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        description: String,
        categories: serde_json::Value,
    }

    let req: Req = data.params()?;

    #[derive(AsChangeset)]
    #[table_name = "mandels"]
    struct UpdateMandela {
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        description: String,
        update_ts: NaiveDateTime,
    }

    let update_mandela = UpdateMandela {
        title_mode: req.title_mode,
        title: req.title,
        what: req.what,
        before: req.before,
        after: req.after,
        description: req.description,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(mandels.filter(mandels::id.eq(req.id)))
        .set(&update_mandela)
        .execute(&data.db.conn)?;

    let category_numbers: Vec<i16> = serde_json::from_value(req.categories).unwrap();
    update_categories(&data.db.conn, req.id, category_numbers)?;

    Ok(None)
}

fn get_poll(db: &db::Db, mandela_id: Id) -> Vec<Votes> {
    use diesel::dsl::*;

    sql_query(
        "SELECT vote, COUNT (*) as count FROM votes AS v
    JOIN mandels AS m ON m.id = v.mandela_id
    WHERE m.id = $1
    GROUP BY vote",
    )
    .bind::<Int4, _>(mandela_id)
    .load::<Votes>(&db.conn)
    .unwrap()
}

// mandela.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    use crate::model::schema::marks;
    use crate::model::schema::marks::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req: Req = data.params()?;

    #[derive(Queryable, Serialize)]
    pub struct Mandela {
        id: Id,
        title: String,
        title_mode: i32,
        description: String,
        user_id: Id,
        user_name: String,
        create_ts: NaiveDateTime,
        update_ts: NaiveDateTime,
        what: String,
        before: String,
        after: String,
        mark_ts: Option<NaiveDateTime>,
    }

    let mandela_record = mandels
        .inner_join(users)
        .left_join(
            marks.on(marks::user_id
                .eq(data.user.id)
                .and(marks::mandela_id.eq(mandels::id))),
        )
        .select((
            mandels::id,
            title,
            title_mode,
            description,
            mandels::user_id,
            users::name,
            mandels::create_ts,
            mandels::update_ts,
            what,
            before,
            after,
            marks::create_ts.nullable(),
        ))
        .filter(mandels::id.eq(req.id))
        .first::<Mandela>(&data.db.conn);

    let mandela: Mandela;

    if let Ok(m) = mandela_record {
        mandela = m;
    } else {
        return Err(api::make_error(api::error::RECORD_NOT_FOUND));
    }

    let mandela_votes = get_poll(&data.db, req.id);

    use crate::model::schema::votes;
    use crate::model::schema::votes::dsl::*;

    let mut mandela_vote: Option<i16> = None;

    if data.user.code != types::UserCode::Anonym {
        mandela_vote = votes
            .select(votes::vote)
            .filter(
                votes::mandela_id
                    .eq(req.id)
                    .and(votes::user_id.eq(data.user.id)),
            )
            .get_result::<i16>(&data.db.conn)
            .optional()?;
    };

    use crate::model::schema::categories;
    use crate::model::schema::categories::dsl::*;

    let category_numbers = categories
        .select(categories::number)
        .filter(categories::mandela_id.eq(req.id))
        .load(&data.db.conn)?;

    #[derive(Serialize)]
    struct MandelaResp {
        mandela: Mandela,
        votes: Vec<Votes>,
        vote: Option<i16>,
        categories: Vec<i16>,
    }

    let resp = MandelaResp {
        mandela: mandela,
        votes: mandela_votes,
        vote: mandela_vote,
        categories: category_numbers,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// mandela.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    use crate::model::schema::categories;
    use crate::model::schema::categories::dsl::*;
    use crate::model::schema::comments;
    use crate::model::schema::comments::dsl::*;
    use crate::model::schema::mandels;
    use crate::model::schema::mandels::dsl::*;
    use crate::model::schema::marks;
    use crate::model::schema::marks::dsl::*;
    use crate::model::schema::users;
    use crate::model::schema::users::dsl::*;
    use crate::model::schema::votes;
    use crate::model::schema::votes::dsl::*;
    use diesel::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        offset: i64,
        limit: i64,
        filter: Option<i8>,
        category: Option<i16>,
        sort: i8,
    }

    let req: Req = data.params()?;

    #[derive(Queryable)]
    struct Mandela {
        id: Id,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        create_ts: NaiveDateTime,
        user_name: Option<String>,
        user_id: Id,
        mark_ts: Option<NaiveDateTime>,
    }

    const SHOW_ALL: i8 = 0;
    const SHOW_NEW: i8 = 1;
    const SHOW_MINE: i8 = 2;
    const SHOW_POLL: i8 = 3;
    const SHOW_CATEGORY: i8 = 4;

    let filter = if let Some(i) = req.filter {
        i
    } else {
        SHOW_ALL
    };

    let mut query = mandels
        .inner_join(users)
        .left_join(
            marks.on(marks::user_id
                .eq(data.user.id)
                .and(marks::mandela_id.eq(mandels::id))),
        )
        .left_join(
            votes.on(votes::user_id
                .eq(data.user.id)
                .and(votes::mandela_id.eq(mandels::id))),
        )
        .left_join(categories.on(categories::mandela_id.eq(mandels::id)))
        .left_join(comments.on(comments::mandela_id.eq(mandels::id)))
        .select((
            mandels::id,
            title_mode,
            title,
            what,
            before,
            after,
            mandels::create_ts,
            users::name.nullable(),
            users::id,
            marks::create_ts.nullable(),
        ))
        .into_boxed();

    if filter == SHOW_NEW {
        query = query.filter(marks::create_ts.is_null())
    } else if filter == SHOW_MINE {
        query = query.filter(mandels::user_id.eq(data.user.id))
    } else if filter == SHOW_POLL {
        query = query.filter(votes::create_ts.is_null())
    } else if filter == SHOW_CATEGORY {
        query = query.filter(categories::number.eq(req.category.unwrap()));
    }

    query = query.group_by((
        mandels::id,
        users::name,
        users::id,
        marks::create_ts,
        votes::create_ts,
    ));

    const SORT_MANDELA: i8 = 0;
    const SORT_COMMENT: i8 = 1;

    if req.sort == SORT_MANDELA {
        query = query.order(mandels::id.desc());
    } else if req.sort == SORT_COMMENT {
        query = query.order(max(comments::create_ts).desc().nulls_last());
    }

    let list = query
        .offset(req.offset)
        .limit(req.limit)
        .load::<Mandela>(&data.db.conn)?;

    #[derive(Serialize)]
    struct MandelaResp {
        id: Id,
        title_mode: i32,
        title: String,
        what: String,
        before: String,
        after: String,
        create_ts: NaiveDateTime,
        user_name: Option<String>,
        user_id: Id,
        comment_count: i64,
        mark_ts: Option<NaiveDateTime>,
        votes: Vec<Votes>,
    }

    let mut mandels_resp: Vec<MandelaResp> = Vec::new();

    for elem in list {
        let comment_count: i64 = comments
            .filter(comments::mandela_id.eq(elem.id))
            .select(count_star())
            .first(&data.db.conn)?;

        let mandela_votes = get_poll(&data.db, elem.id);

        let mandela_resp = MandelaResp {
            id: elem.id,
            title_mode: elem.title_mode,
            title: elem.title,
            what: elem.what,
            before: elem.before,
            after: elem.after,
            create_ts: elem.create_ts,
            user_name: elem.user_name,
            user_id: elem.user_id,
            comment_count: comment_count,
            mark_ts: elem.mark_ts,
            votes: mandela_votes,
        };

        mandels_resp.push(mandela_resp);
    }

    let total_count: i64 = mandels.select(count_star()).first(&data.db.conn)?;
    let mut new_count = 0;
    let mut mine_count = 0;
    let mut poll_count = 0;
    let mut category_count = 0;

    if filter == SHOW_CATEGORY {
        category_count = mandels
            .select(count_star())
            .inner_join(categories)
            .filter(number.eq(req.category.unwrap()))
            .first(&data.db.conn)?;
    }

    if data.user.code != types::UserCode::Anonym {
        let mark_count: i64 = marks
            .select(count_star())
            .filter(marks::user_id.eq(data.user.id))
            .first(&data.db.conn)?;
        new_count = total_count - mark_count;

        let vote_count: i64 = votes
            .select(count_star())
            .filter(votes::user_id.eq(data.user.id))
            .first(&data.db.conn)?;

        poll_count = total_count - vote_count;

        mine_count = mandels
            .select(count_star())
            .filter(mandels::user_id.eq(data.user.id))
            .first(&data.db.conn)?;
    }

    #[derive(Serialize)]
    struct Resp {
        total_count: i64,
        new_count: i64,
        mine_count: i64,
        poll_count: i64,
        category_count: i64,
        mandels: Vec<MandelaResp>,
    };

    let resp = serde_json::to_value(&Resp {
        total_count: total_count,
        new_count: new_count,
        mine_count: mine_count,
        poll_count: poll_count,
        category_count: category_count,
        mandels: mandels_resp,
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

    let req: Req = data.params()?;

    use crate::model::schema::mandels::dsl::*;

    diesel::delete(mandels.filter(id.eq_any(req.id))).execute(&data.db.conn)?;
    Ok(None)
}

// mandela.mark
pub fn mark(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Id,
    }

    let req: Req = data.params()?;

    use crate::model::schema::marks;
    use crate::model::schema::marks::dsl::*;

    #[derive(Insertable)]
    #[table_name = "marks"]
    pub struct NewMark {
        mandela_id: Id,
        user_id: Id,
    };

    let new_mark = NewMark {
        mandela_id: req.id,
        user_id: data.user.id,
    };

    diesel::insert_into(marks)
        .values(&new_mark)
        .execute(&data.db.conn)?;
    Ok(None)
}

// mandela.vote
pub fn vote(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        id: Id,
        vote: i16,
    }

    let req: Req = data.params()?;

    #[derive(Insertable, AsChangeset)]
    #[table_name = "votes"]
    pub struct NewVote {
        mandela_id: Id,
        user_id: Id,
        vote: i16,
    };

    let new_vote = NewVote {
        mandela_id: req.id,
        user_id: data.user.id,
        vote: req.vote,
    };

    use crate::model::schema::votes;
    use crate::model::schema::votes::dsl::*;

    let vote_id = votes
        .select(id)
        .filter(mandela_id.eq(req.id).and(user_id.eq(data.user.id)))
        .first::<Id>(&data.db.conn)
        .optional()?;

    if let Some(i) = vote_id {
        diesel::update(votes.filter(votes::id.eq(i)))
            .set(&new_vote)
            .execute(&data.db.conn)?;
    } else {
        diesel::insert_into(votes)
            .values(&new_vote)
            .execute(&data.db.conn)?;
    }

    let votes_count = get_poll(&data.db, req.id);
    let result = serde_json::to_value(&votes_count)?;
    Ok(Some(result))
}

pub fn new_comments(
    db: &db::Db,
    limit: i32,
    offset: i32,
) -> Result<Vec<Comment>, Box<dyn std::error::Error>> {
    let result = diesel::dsl::sql_query(
        "SELECT id, mandela_id, title_mode, title, what, before, after, message, user_id, user_name, create_ts, comment_count
        FROM (SELECT m.id AS mandela_id, m.title_mode, m.title, m.what, m.before, m.after, c.id, c.message, c.user_id, u.name AS user_name, c.create_ts,
            rank() over (PARTITION BY c.mandela_id ORDER BY c.create_ts DESC),
            (SELECT count (*) FROM comments WHERE mandela_id = m.id) as comment_count
        FROM mandels AS m
            LEFT JOIN comments AS c ON c.mandela_id = m.id
            INNER JOIN users AS u ON c.user_id = u.id) AS x
        WHERE x.rank = 1
        ORDER BY x.create_ts DESC
        LIMIT $1
        OFFSET $2",
    )
    .bind::<Int4, _>(limit)
    .bind::<Int4, _>(offset)
    .load::<Comment>(&db.conn)?;

    Ok(result)
}

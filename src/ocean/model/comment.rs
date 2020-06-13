use crate::model::schema::comments;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub user_name: Option<String>,
    pub message: String,
    pub create_ts: NaiveDateTime,
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "comments"]
pub struct NewComment {
    pub mandela_id: i32,
    pub user_id: i32,
    pub message: String,
}

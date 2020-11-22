use crate::model::schema::comments;
use crate::types::Id;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: Id,
    pub user_id: Id,
    pub user_name: String,
    pub message: String,
    pub create_ts: NaiveDateTime,
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "comments"]
pub struct NewComment {
    pub mandela_id: Id,
    pub user_id: Id,
    pub message: String,
}

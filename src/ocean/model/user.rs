use crate::model::schema::users;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub token: String,
    pub group_id: i32,
    pub create_ts: NaiveDateTime,
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: Option<String>,
    pub group_id: i32,
}

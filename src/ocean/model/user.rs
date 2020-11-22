use crate::model::schema::users;
use crate::types::Id;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub token: String,
    pub group_id: Id,
    pub create_ts: NaiveDateTime,
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub group_id: Id,
    pub token: String,
}

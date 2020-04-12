use crate::model::schema::topics;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub create_ts: NaiveDateTime,
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "topics"]
pub struct NewTopic<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

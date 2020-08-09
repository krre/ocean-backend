use crate::model::schema::mandels;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "mandels"]
pub struct NewMandela {
    pub title_mode: i32,
    pub title: String,
    pub what: String,
    pub before: String,
    pub after: String,
    pub description: String,
    pub user_id: i32,
    pub images: serde_json::Value,
    pub videos: serde_json::Value,
    pub links: serde_json::Value,
}

#[derive(AsChangeset)]
#[table_name = "mandels"]
pub struct UpdateMandela {
    pub title_mode: i32,
    pub title: String,
    pub what: String,
    pub before: String,
    pub after: String,
    pub description: String,
    pub images: serde_json::Value,
    pub videos: serde_json::Value,
    pub links: serde_json::Value,
    pub user_id: i32,
    pub update_ts: NaiveDateTime,
}

#[derive(Queryable)]
pub struct MandelaTitle {
    pub id: i32,
    pub title_mode: i32,
    pub title: String,
    pub what: String,
    pub before: String,
    pub after: String,
}

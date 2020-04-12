use std::time::SystemTime;

#[derive(Queryable)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub create_ts: SystemTime,
    pub update_ts: SystemTime,
}

use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct UserGroup {
    pub id: i32,
    pub name: Option<String>,
    pub code: String,
}

use crate::types::Id;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct UserGroup {
    pub id: Id,
    pub name: Option<String>,
    pub code: String,
}

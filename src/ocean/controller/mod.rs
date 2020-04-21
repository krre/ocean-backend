use crate::db;
use serde_json;

pub mod topic;
pub mod user;

pub trait Controller {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Option<serde_json::Value>;
}

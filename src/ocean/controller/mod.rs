use crate::db;
use serde_json;

pub mod topic;

pub trait Controller {
    fn new() -> Self;
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Option<serde_json::Value>;
}

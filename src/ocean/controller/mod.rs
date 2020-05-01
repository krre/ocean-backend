use crate::db;
use serde_json;
use std::error::Error;

pub mod topic;
pub mod user;

pub trait Controller {
    fn exec(
        &self,
        db: &db::Db,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>>;
}

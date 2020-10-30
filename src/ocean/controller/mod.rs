use crate::config;
use crate::db;
use crate::model;
use crate::types;

pub mod comment;
pub mod mandela;
pub mod rating;
pub mod search;
pub mod user;

pub type RequestResult = Result<Option<serde_json::Value>, Box<dyn std::error::Error>>;
pub type RequestHandler = fn(RequestData) -> RequestResult;

pub struct RequestData {
    db: db::Db,
    user: types::User,
    params: Option<serde_json::Value>,
}

impl RequestData {
    pub fn new(db: db::Db, user: types::User, params: Option<serde_json::Value>) -> Self {
        Self { db, user, params }
    }
}

pub fn format_mandela_title(mandela_title: model::mandela::MandelaTitle) -> String {
    const TITLE_MODE_SIMPLE: i32 = 0;
    const TITLE_MODE_COMPLEX: i32 = 1;

    let title = if mandela_title.title_mode == TITLE_MODE_SIMPLE {
        mandela_title.title.clone()
    } else if mandela_title.title_mode == TITLE_MODE_COMPLEX {
        mandela_title.what.clone() + ": " + &mandela_title.before + " / " + &mandela_title.after
    } else {
        "Неизвестная мандела".into()
    };

    format!(
        "<a href='{}/mandela/{}'>{}</a>",
        config::CONFIG.frontend.domen,
        mandela_title.id,
        title
    )
}

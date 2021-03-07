use crate::config;
use crate::db;
use crate::types;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod activity;
pub mod comment;
pub mod feed;
pub mod forum;
pub mod mandela;
pub mod rating;
pub mod search;
pub mod user;

pub type RequestResult = Result<Option<serde_json::Value>, Box<dyn std::error::Error>>;
pub type RequestHandler = fn(RequestData) -> RequestResult;

#[derive(Deserialize)]
pub struct RequestId {
    id: types::Id,
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Controller error: {}", self.message)
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

pub struct RequestData {
    db: db::Db,
    user: types::User,
    params: Option<serde_json::Value>,
}

impl RequestData {
    pub fn new(db: db::Db, user: types::User, params: Option<serde_json::Value>) -> Self {
        Self { db, user, params }
    }

    pub fn params<T: DeserializeOwned>(&self) -> Result<T, Box<dyn std::error::Error>> {
        if let Some(p) = &self.params {
            Ok(serde_json::from_value::<T>((*p).clone())?)
        } else {
            Err(Box::new(Error::new(String::from("Parameters is absent"))))
        }
    }
}

pub fn format_mandela_title(mandela_title: mandela::MandelaTitle) -> String {
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

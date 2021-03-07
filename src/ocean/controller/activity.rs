use crate::controller::forum;
use crate::controller::mandela;
use crate::controller::*;
use serde::Deserialize;
use serde::Serialize;

// activity.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        limit: i32,
    }

    let req: Req = data.params()?;

    #[derive(Serialize)]
    struct Resp {
        comments: Vec<mandela::Comment>,
        topics: Vec<forum::Topic>,
    };

    let topics = forum::new_topics(&data.db, req.limit, 0)?;
    let comments = mandela::new_comments(&data.db, req.limit, 0)?;

    let resp = serde_json::to_value(&Resp { comments, topics })?;

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

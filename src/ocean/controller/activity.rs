use crate::controller::forum;
use crate::controller::mandela;
use crate::controller::*;
use serde::{Deserialize, Serialize};

// activity.getAll
pub fn get_all(mut data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        limit: i32,
    }

    let req: Req = data.params()?;

    #[derive(Serialize)]
    struct Resp {
        comments: Vec<mandela::Comment>,
        topics: Vec<forum::Topic>,
    }

    let topics = forum::new_topics(&mut data.db, req.limit, 0)?;
    let comments = mandela::new_comments(&mut data.db, req.limit, 0)?;

    let resp = Resp { comments, topics };
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

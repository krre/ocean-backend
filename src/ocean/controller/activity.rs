use crate::controller::forum;
use crate::controller::*;
use serde::Deserialize;
use serde::Serialize;

// activity.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        limit: i32,
    }

    let req = serde_json::from_value::<Req>(data.params.unwrap())?;

    #[derive(Serialize)]
    struct Resp {
        topics: Vec<forum::Topic>,
    };

    let topics = forum::new_topics(&data.db, req.limit, 0)?;
    let resp = serde_json::to_value(&Resp { topics })?;
    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

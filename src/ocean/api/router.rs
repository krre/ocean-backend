use crate::api;
use crate::api::authorizer;
use crate::api::user_cache;
use crate::controller;
use crate::db;
use crate::json_rpc;
use crate::types;
use http_body_util::{BodyExt, Full};
use hyper::body::Buf;
use hyper::body::Bytes;
use hyper::{Method, Request, Response, StatusCode, body::Incoming as IncomingBody, header};
use log::{error, info};
use std::collections::HashMap;
use std::net::SocketAddr;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type ResponseResult = Result<Response<BoxBody>>;

struct Rh(controller::RequestHandler);

lazy_static! {
    static ref METHODS: HashMap<String, Rh> = {
        let mut m = HashMap::new();
        m.insert("ping".to_string(), Rh(controller::ping));
        m.insert(
            "mandela.create".to_string(),
            Rh(controller::mandela::create),
        );
        m.insert(
            "mandela.update".to_string(),
            Rh(controller::mandela::update),
        );
        m.insert(
            "mandela.getOne".to_string(),
            Rh(controller::mandela::get_one),
        );
        m.insert(
            "mandela.getAll".to_string(),
            Rh(controller::mandela::get_all),
        );
        m.insert(
            "mandela.delete".to_string(),
            Rh(controller::mandela::delete),
        );
        m.insert("mandela.mark".to_string(), Rh(controller::mandela::mark));
        m.insert("mandela.vote".to_string(), Rh(controller::mandela::vote));
        m.insert(
            "mandela.getVoteUsers".to_string(),
            Rh(controller::mandela::get_vote_users),
        );
        m.insert(
            "mandela.updateTrash".to_string(),
            Rh(controller::mandela::update_trash),
        );
        m.insert(
            "user.getNextId".to_string(),
            Rh(controller::user::get_next_id),
        );
        m.insert("user.create".to_string(), Rh(controller::user::create));
        m.insert("user.delete".to_string(), Rh(controller::user::delete));
        m.insert("user.auth".to_string(), Rh(controller::user::auth));
        m.insert("user.logout".to_string(), Rh(controller::user::logout));
        m.insert("user.getOne".to_string(), Rh(controller::user::get_one));
        m.insert("user.update".to_string(), Rh(controller::user::update));
        m.insert(
            "user.updateToken".to_string(),
            Rh(controller::user::update_token),
        );
        m.insert(
            "user.updateProfile".to_string(),
            Rh(controller::user::update_profile),
        );
        m.insert(
            "comment.create".to_string(),
            Rh(controller::comment::create),
        );
        m.insert(
            "comment.getAll".to_string(),
            Rh(controller::comment::get_all),
        );
        m.insert(
            "comment.update".to_string(),
            Rh(controller::comment::update),
        );
        m.insert(
            "comment.delete".to_string(),
            Rh(controller::comment::delete),
        );
        m.insert("like.create".to_string(), Rh(controller::like::create));
        m.insert("like.delete".to_string(), Rh(controller::like::delete));
        m.insert("like.getUsers".to_string(), Rh(controller::like::get_users));
        m.insert("search.getAll".to_string(), Rh(controller::search::get_all));
        m.insert(
            "rating.getMandels".to_string(),
            Rh(controller::rating::get_mandels),
        );
        m.insert(
            "rating.getUsers".to_string(),
            Rh(controller::rating::get_users),
        );
        m.insert("forum.getAll".to_string(), Rh(controller::forum::get_all));
        m.insert("forum.getNew".to_string(), Rh(controller::forum::get_new));
        m.insert(
            "forum.category.create".to_string(),
            Rh(controller::forum::category::create),
        );
        m.insert(
            "forum.category.getOne".to_string(),
            Rh(controller::forum::category::get_one),
        );
        m.insert(
            "forum.category.update".to_string(),
            Rh(controller::forum::category::update),
        );
        m.insert(
            "forum.category.delete".to_string(),
            Rh(controller::forum::category::delete),
        );
        m.insert(
            "forum.section.create".to_string(),
            Rh(controller::forum::section::create),
        );
        m.insert(
            "forum.section.getAll".to_string(),
            Rh(controller::forum::section::get_all),
        );
        m.insert(
            "forum.section.getOne".to_string(),
            Rh(controller::forum::section::get_one),
        );
        m.insert(
            "forum.section.update".to_string(),
            Rh(controller::forum::section::update),
        );
        m.insert(
            "forum.section.delete".to_string(),
            Rh(controller::forum::section::delete),
        );
        m.insert(
            "forum.topic.getAll".to_string(),
            Rh(controller::forum::topic::get_all),
        );
        m.insert(
            "forum.topic.getOne".to_string(),
            Rh(controller::forum::topic::get_one),
        );
        m.insert(
            "forum.topic.create".to_string(),
            Rh(controller::forum::topic::create),
        );
        m.insert(
            "forum.topic.update".to_string(),
            Rh(controller::forum::topic::update),
        );
        m.insert(
            "forum.topic.delete".to_string(),
            Rh(controller::forum::topic::delete),
        );
        m.insert(
            "forum.topic.vote".to_string(),
            Rh(controller::forum::topic::vote),
        );
        m.insert(
            "forum.topic.getVoteUsers".to_string(),
            Rh(controller::forum::topic::get_vote_users),
        );
        m.insert(
            "forum.post.getAll".to_string(),
            Rh(controller::forum::post::get_all),
        );
        m.insert(
            "forum.post.getOne".to_string(),
            Rh(controller::forum::post::get_one),
        );
        m.insert(
            "forum.post.create".to_string(),
            Rh(controller::forum::post::create),
        );
        m.insert(
            "forum.post.update".to_string(),
            Rh(controller::forum::post::update),
        );
        m.insert(
            "forum.post.delete".to_string(),
            Rh(controller::forum::post::delete),
        );
        m.insert(
            "activity.getAll".to_string(),
            Rh(controller::activity::get_all),
        );
        m.insert("feed.getAll".to_string(), Rh(controller::feed::get_all));
        m
    };
}

pub async fn route(req: Request<IncomingBody>, addr: SocketAddr) -> ResponseResult {
    if req.method() != Method::POST || req.uri().path() != "/api" {
        return bad_request(req);
    }

    let query;
    if let Some(q) = req.uri().query() {
        query = q;
    } else {
        return bad_request(req);
    };

    let url_params = url::form_urlencoded::parse(query.as_bytes());
    let hash_params: HashMap<_, _> = url_params.into_owned().collect();

    let token;

    if let Some(t) = hash_params.get("token") {
        token = t;
    } else {
        return bad_request(req);
    }

    let user;

    if let Some(u) = user_cache::get(token) {
        user = u;
    } else {
        return unauthorized(token);
    }

    let user_id = user.id;
    let user_name = user.name.clone();
    let whole_body = req.collect().await?.aggregate();
    let bytes = whole_body.chunk();
    let raw_req = String::from_utf8_lossy(bytes);

    info!(
        "[REQUEST] {} ({}: {}) {}",
        addr.ip(),
        user_id,
        user.name,
        raw_req
    );

    let json_rpc_req = serde_json::from_slice::<json_rpc::Request>(bytes);

    let json_rpc_resp = if let Ok(r) = json_rpc_req {
        exec(user, r)
    } else {
        json_rpc::response::Response {
            error: Some(json_rpc::Error::from_api_error(&api::Error::new(
                api::error::PARSE_ERROR,
                Some(json_rpc_req.err().unwrap().to_string()),
            ))),
            ..Default::default()
        }
    };

    let raw_resp = serde_json::to_string(&json_rpc_resp).unwrap();
    info!(
        "[RESPONSE] {} ({}: {}) {}",
        addr.ip(),
        user_id,
        user_name,
        raw_resp
    );

    let mut response = Response::builder().body(full(raw_resp)).unwrap();
    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        header::HeaderValue::from_static("*"),
    );

    Ok(response)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn bad_request(req: Request<IncomingBody>) -> ResponseResult {
    info!(
        "Bad request: method: {}, URL: {}",
        req.method().as_str(),
        req.uri().path()
    );

    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(full("Bad request"))
        .unwrap())
}

fn unauthorized(token: &str) -> ResponseResult {
    info!("Unauthorized: token: {}", token);

    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(full("Unauthorized"))
        .unwrap())
}

fn exec(user: types::User, req: json_rpc::Request) -> json_rpc::Response {
    let mut resp = json_rpc::Response::default();

    if let Some(id) = req.id {
        resp.id = id;
    }

    let method = req.method;
    resp.method = method.clone();

    if !authorizer::authorize(&method, &user.code) {
        resp.error = Some(json_rpc::Error::from_api_error(&api::Error::new(
            api::error::ACCESS_DENIED,
            None,
        )));
        return resp;
    }

    if user.blocked && method != "user.logout" {
        resp.error = Some(json_rpc::Error::from_api_error(&api::Error::new(
            api::error::ACCOUNT_BLOCKED,
            None,
        )));
        return resp;
    }

    match METHODS.get(&method) {
        Some(func) => {
            let db = db::Db::new();
            let data = controller::RequestData::new(db, user, req.params);
            let result = func.0(data);

            match result {
                Ok(r) => resp.result = r,
                Err(e) => {
                    let api_err = e.downcast_ref::<api::error::Error>();
                    if let Some(i) = api_err {
                        resp.error = Some(json_rpc::Error::from_api_error(i));
                    } else {
                        error!("{}", e);
                        let server_err =
                            api::error::Error::new(api::error::INTERNAL_SERVER_ERROR, None);
                        resp.error = Some(json_rpc::Error::from_api_error(&server_err));
                    }
                }
            };
        }
        None => {
            let server_err = api::error::Error::new(api::error::METHOD_NOT_FOUND, Some(method));
            resp.error = Some(json_rpc::Error::from_api_error(&server_err));
        }
    }
    resp
}

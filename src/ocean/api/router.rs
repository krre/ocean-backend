use crate::api;
use crate::api::authorizer;
use crate::api::user_cache;
use crate::controller;
use crate::db;
use crate::json_rpc;
use crate::types;
use hyper::body;
use hyper::header;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::{error, info};
use std::collections::HashMap;
use std::net::SocketAddr;
use url;

type ResponseResult = Result<Response<Body>, hyper::Error>;

lazy_static! {
    static ref METHODS: HashMap<String, Rh> = {
        let mut m = HashMap::new();
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
            "user.getNextId".to_string(),
            Rh(controller::user::get_next_id),
        );
        m.insert("user.create".to_string(), Rh(controller::user::create));
        m.insert("user.auth".to_string(), Rh(controller::user::auth));
        m.insert("user.logout".to_string(), Rh(controller::user::logout));
        m.insert("user.getOne".to_string(), Rh(controller::user::get_one));
        m.insert("user.update".to_string(), Rh(controller::user::update));
        m.insert(
            "user.updateToken".to_string(),
            Rh(controller::user::update_token),
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
        m.insert(
            "search.getByContent".to_string(),
            Rh(controller::search::get_by_content),
        );
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

struct Rh(controller::RequestHandler);

pub async fn route(req: Request<Body>, addr: SocketAddr) -> ResponseResult {
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
    // let address = req.remote

    let whole_body = body::to_bytes(req).await?;
    let bytes = whole_body.as_ref();
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
        if !authorizer::authorize(&r.method, &user.code) {
            return forbidden(&r.method, &user.code);
        }

        exec(user, r)
    } else {
        let mut resp = json_rpc::Response::default();
        resp.error = Some(json_rpc::Error::from_api_error(&api::Error::new(
            api::error::PARSE_ERROR,
            Some(json_rpc_req.err().unwrap().to_string()),
        )));
        resp
    };

    let raw_resp = serde_json::to_string(&json_rpc_resp).unwrap();
    info!(
        "[RESPONSE] {} ({}: {}) {}",
        addr.ip(),
        user_id,
        user_name,
        raw_resp
    );

    let mut response = Response::new(Body::from(raw_resp));
    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        header::HeaderValue::from_static("*"),
    );

    Ok(response)
}

fn bad_request(req: Request<Body>) -> ResponseResult {
    info!(
        "Bad request: method: {}, URL: {}",
        req.method().as_str(),
        req.uri().path()
    );

    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Bad request"))
        .unwrap())
}

fn unauthorized(token: &String) -> ResponseResult {
    info!("Unauthorized: token: {}", token);

    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized"))
        .unwrap())
}

fn forbidden(method: &String, user_code: &types::UserCode) -> ResponseResult {
    info!("Forbidden: method: {} user code: {:?}", method, user_code);

    Ok(Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from("Forbidden"))
        .unwrap())
}

fn exec(user: types::User, req: json_rpc::Request) -> json_rpc::Response {
    let mut resp = json_rpc::Response::default();

    if let Some(id) = req.id {
        resp.id = id;
    }

    let method = req.method;
    resp.method = method.clone();

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

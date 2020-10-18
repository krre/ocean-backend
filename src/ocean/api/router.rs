use crate::api;
use crate::controller;
use crate::db;
use crate::json_rpc;
use hyper::body;
use hyper::body::Buf;
use hyper::header;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::{error, info};
use std::collections::HashMap;

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
        m.insert("user.create".to_string(), Rh(controller::user::create));
        m.insert("user.auth".to_string(), Rh(controller::user::auth));
        m.insert("user.getOne".to_string(), Rh(controller::user::get_one));
        m.insert("user.update".to_string(), Rh(controller::user::update));
        m.insert(
            "user.changePassword".to_string(),
            Rh(controller::user::change_password),
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
            "search.getById".to_string(),
            Rh(controller::search::get_by_id),
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
        m.insert(
            "forumCategory.create".to_string(),
            Rh(controller::forum::category::create),
        );
        m
    };
}

struct Rh(controller::RequestHandler);

pub async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() != Method::POST || req.uri().path() != "/api" {
        info!(
            "Bad request: method: {}, URL: {}",
            req.method().as_str(),
            req.uri().path()
        );
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Bad request"))
            .unwrap());
    }

    let whole_body = body::aggregate(req).await?;
    let bytes = whole_body.bytes();
    let raw_req = String::from_utf8(bytes.to_vec()).unwrap();

    info!("Request: {}", raw_req);

    let json_rpc_req = serde_json::from_slice::<json_rpc::Request>(bytes);

    let json_rpc_resp = if let Ok(r) = json_rpc_req {
        exec(r)
    } else {
        let mut resp = json_rpc::Response::default();
        resp.error = Some(json_rpc::Error::from_api_error(&api::Error::new(
            api::error::PARSE_ERROR,
            Some(json_rpc_req.err().unwrap().to_string()),
        )));
        resp
    };

    let raw_resp = serde_json::to_string(&json_rpc_resp).unwrap();
    info!("Response: {}", raw_resp);

    let mut response = Response::new(Body::from(raw_resp));
    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        header::HeaderValue::from_static("*"),
    );

    Ok(response)
}

fn exec(req: json_rpc::Request) -> json_rpc::Response {
    let mut resp = json_rpc::Response::default();

    if let Some(id) = req.id {
        resp.id = id;
    }

    let method = req.method;
    resp.method = method.clone();

    match METHODS.get(&method) {
        Some(func) => {
            let db = db::Db::new();
            let data = controller::RequestData::new(db, req.params);
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

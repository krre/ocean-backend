use hyper::body;
use hyper::body::Buf;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde_json;

pub async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() != Method::POST || req.uri().path() != "/dive" {
        println!(
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

    println!("Request: {}", raw_req);

    let value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    Ok(Response::new(exec(&value)))
}

fn exec(req: &serde_json::Value) -> Body {
    let method = &req["method"];
    let method: Vec<&str> = method.as_str().unwrap().split('.').collect();
    let controller = method[0];
    let method = method[1];
    println!("{} {}", controller, method);
    Body::from("hello, world!")
}

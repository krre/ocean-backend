use hyper::{Body, Method, Request, Response, StatusCode};

pub fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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

    Ok(Response::new(Body::from("hello, world!")))
}

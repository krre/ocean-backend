use hyper::{Body, Method, Request, Response, StatusCode};

pub fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{}", req.uri().path());

    if req.method() != Method::POST || req.uri().path() != "/dive" {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Bad request"))
            .unwrap());
    }

    Ok(Response::new(Body::from("hello, world!")))
}

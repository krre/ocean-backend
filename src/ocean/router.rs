use hyper::{Body, Request, Response};

pub fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{}", req.uri().path());
    Ok(Response::new(Body::from("hello, world!")))
}

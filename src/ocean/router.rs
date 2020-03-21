use hyper::{Body, Request, Response};

pub struct Router {}

impl Router {
    pub fn new() -> Router {
        Router {}
    }

    pub fn route(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        println!("{}", req.uri().path());
        Ok(Response::new(Body::from("hello, world!")))
    }
}

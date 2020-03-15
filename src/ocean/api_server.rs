use crate::router;
use hyper::service::{make_service_fn, service_fn};
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;

pub struct ApiServer {
    port: u16,
    router: router::Router,
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world!")))
}

impl ApiServer {
    pub fn new(port: u16) -> ApiServer {
        let router = router::Router::new();
        ApiServer { port, router }
    }

    pub async fn listen(&self) {
        println!("API server listen on port {}", self.port);

        let addr = ([127, 0, 0, 1], self.port).into();

        let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }));

        if let Err(e) = serve_future.await {
            eprintln!("server error: {}", e);
        }
    }
}

use crate::router;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

pub struct ApiServer {
    port: u16,
}

impl ApiServer {
    pub fn new(port: u16) -> ApiServer {
        ApiServer { port }
    }

    pub async fn listen(&self) {
        let addr = ([127, 0, 0, 1], self.port).into();

        let service = make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(move |req| async { router::route(req) }))
        });

        let server = Server::bind(&addr).serve(service);

        println!("API server listen on port {}", self.port);

        if let Err(e) = server.await {
            eprintln!("API server error: {}", e);
        }
    }
}

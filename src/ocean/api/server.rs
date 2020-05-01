use crate::config;
use crate::router;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server as HyperServer;

pub struct Server;

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn listen(&self) {
        let port = config::CONFIG.server.port;
        let addr = ([127, 0, 0, 1], port).into();

        let service = make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(move |req| router::route(req)))
        });

        let server = HyperServer::bind(&addr).serve(service);

        println!("API server listen on port {}", port);

        if let Err(e) = server.await {
            eprintln!("API server error: {}", e);
        }
    }
}

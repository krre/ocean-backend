use hyper::service::{make_service_fn, service_fn};
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;

pub struct ApiServer {
    port: u16,
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}

impl ApiServer {
    pub fn new(port: u16) -> ApiServer {
        ApiServer { port }
    }

    pub async fn listen(&self) {
        println!("API server listen on port {}", self.port);

        let addr = ([127, 0, 0, 1], self.port).into();

        // Create a server bound on the provided address
        let serve_future = Server::bind(&addr)
            // Serve requests using our `async serve_req` function.
            // `serve` takes a closure which returns a type implementing the
            // `Service` trait. `service_fn` returns a value implementing the
            // `Service` trait, and accepts a closure which goes from request
            // to a future of the response.
            .serve(make_service_fn(|_| async {
                {
                    Ok::<_, hyper::Error>(service_fn(serve_req))
                }
            }));

        // Wait for the server to complete serving or exit with an error.
        // If an error occurred, print it to stderr.
        if let Err(e) = serve_future.await {
            eprintln!("server error: {}", e);
        }
    }
}

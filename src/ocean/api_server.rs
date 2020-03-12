pub struct ApiServer {
    port: u16,
}

impl ApiServer {
    pub fn new(port: u16) -> ApiServer {
        ApiServer { port }
    }

    pub fn listen(&self) {
        println!("API server listen on port {}", self.port);
    }
}

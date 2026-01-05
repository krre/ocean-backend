use super::router;
use crate::config;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::info;
use rustls::ServerConfig;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::sync;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

pub struct ApiServer;

impl ApiServer {
    pub fn new() -> Self {
        Self
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let port = config::CONFIG.server.port;
        let addr = format!("0.0.0.0:{}", port);

        let tls_cfg = {
            let certs = CertificateDer::pem_file_iter(config::CONFIG.server.ssl.cert.as_str())
                .unwrap()
                .map(|cert| cert.unwrap())
                .collect();

            let private_key =
                PrivateKeyDer::from_pem_file(config::CONFIG.server.ssl.key.as_str()).unwrap();

            let config = ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(certs, private_key)
                .expect("Bad certificate or private key");
            sync::Arc::new(config)
        };

        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        let listener = TcpListener::bind(&addr).await?;
        info!("API server listen on port {}", port);

        loop {
            let (stream, addr) = listener.accept().await?;
            let acceptor = tls_acceptor.clone();

            tokio::task::spawn(async move {
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("TLS error: {:?}", e);
                        return;
                    }
                };

                let io = TokioIo::new(tls_stream);

                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(move |req| router::route(req, addr)))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

impl Default for ApiServer {
    fn default() -> Self {
        Self::new()
    }
}

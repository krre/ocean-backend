use super::router;
use crate::config;
use core::task::{Context, Poll};
use futures_util::{future::TryFutureExt, stream::Stream, StreamExt, TryStreamExt};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use log::info;
use rustls::internal::pemfile;
use std::pin::Pin;
use std::{fs, io, sync};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

#[derive(Default)]
pub struct ApiServer;

impl ApiServer {
    pub fn new() -> Self {
        Self
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let port = config::CONFIG.server.port;
        // let addr = ([0, 0, 0, 0], port).into();
        let addr = format!("0.0.0.0:{}", port);

        let tls_cfg = {
            let certs = load_certs(config::CONFIG.server.ssl.cert.as_str())?;
            let key = load_private_key(config::CONFIG.server.ssl.key.as_str())?;
            // Do not use client certificate authentication.
            let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            // Select a certificate to use.
            cfg.set_single_cert(certs, key)
                .map_err(|e| error(format!("{}", e)))?;
            // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
            cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
            sync::Arc::new(cfg)
        };

        // Create a TCP listener via tokio.
        let mut tcp = TcpListener::bind(&addr).await?;
        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        // Prepare a long-running future stream to accept and serve cients.
        let incoming_tls_stream = tcp
            .incoming()
            .map_err(|e| error(format!("Incoming failed: {:?}", e)))
            .and_then(move |s| {
                tls_acceptor.accept(s).map_err(|e| {
                    println!("[!] Voluntary server halt due to client-connection error...");
                    // Errors could be handled here, instead of server aborting.
                    // Ok(None)
                    error(format!("TLS Error: {:?}", e))
                })
            })
            .boxed();

        let service =
            make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router::route)) });

        // let server = hyper::Server::bind(&addr).serve(service);
        let server = Server::builder(HyperAcceptor {
            acceptor: incoming_tls_stream,
        })
        .serve(service);

        info!("API server listen on port {}", port);

        server.await?;
        Ok(())
    }
}

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

// Load public certificate from file.
fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    pemfile::certs(&mut reader).map_err(|_| error("failed to load certificate".into()))
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;
    if keys.len() != 1 {
        return Err(error("expected a single private key".into()));
    }
    Ok(keys[0].clone())
}

struct HyperAcceptor<'a> {
    acceptor: Pin<Box<dyn Stream<Item = Result<TlsStream<TcpStream>, io::Error>> + 'a>>,
}

impl hyper::server::accept::Accept for HyperAcceptor<'_> {
    type Conn = TlsStream<TcpStream>;
    type Error = io::Error;

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        Pin::new(&mut self.acceptor).poll_next(cx)
    }
}

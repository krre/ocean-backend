use super::router;
use crate::config;
use core::task::{Context, Poll};
use futures_util::{future::TryFutureExt, stream::Stream, StreamExt, TryStreamExt};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use log::info;
use rustls::internal::pemfile;
use std::pin::Pin;
use std::{fs, io, sync};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use tokio_stream::wrappers::TcpListenerStream;

pub struct ApiServer;

impl ApiServer {
    pub fn new() -> Self {
        Self
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let port = config::CONFIG.server.port;
        let addr = format!("0.0.0.0:{}", port);

        let tls_cfg = {
            let certs = load_certs(config::CONFIG.server.ssl.cert.as_str())?;
            let key = load_private_key(config::CONFIG.server.ssl.key.as_str())?;

            let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            cfg.set_single_cert(certs, key)
                .map_err(|e| error(format!("{}", e)))?;
            cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
            sync::Arc::new(cfg)
        };

        let tcp = TcpListener::bind(&addr).await?;
        let stream = TcpListenerStream::new(tcp);
        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        let incoming_tls_stream = stream
            .map_err(|e| error(format!("Incoming failed: {:?}", e)))
            .and_then(move |s| {
                tls_acceptor
                    .accept(s)
                    .map_err(|e| error(format!("TLS Error: {:?}", e)))
            })
            .filter(|i| futures_util::future::ready(i.is_ok())) // Need to filter out errors as they will stop server to accept connections
            .boxed();

        let service = make_service_fn(
            move |conn: &tokio_rustls::server::TlsStream<tokio::net::TcpStream>| {
                let (stream, _) = conn.get_ref();
                let addr = stream.peer_addr().unwrap();
                async move {
                    let addr = addr.clone();
                    Ok::<_, hyper::Error>(service_fn(move |req| router::route(req, addr)))
                }
            },
        );

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

fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    pemfile::certs(&mut reader).map_err(|_| error("failed to load certificate".into()))
}

fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

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

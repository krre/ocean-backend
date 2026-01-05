use super::router;
use crate::config;
use core::task::{Context, Poll};
use futures_util::{future::TryFutureExt, stream::Stream, StreamExt, TryStreamExt};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use log::info;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::pin::Pin;
use std::{io, sync};
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
            let certs = CertificateDer::pem_file_iter(config::CONFIG.server.ssl.cert.as_str())
                .unwrap()
                .map(|cert| cert.unwrap())
                .collect();

            let private_key =
                PrivateKeyDer::from_pem_file(config::CONFIG.server.ssl.key.as_str()).unwrap();

            let config = ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(certs, private_key)
                .expect("bad certificate / private key");
            sync::Arc::new(config)
        };

        let tcp = TcpListener::bind(&addr).await?;
        let stream = TcpListenerStream::new(tcp);
        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        let incoming_tls_stream = stream
            .map_err(|e| error(&format!("Incoming failed: {:?}", e)))
            .and_then(move |s| {
                tls_acceptor
                    .accept(s)
                    .map_err(|e| error(&format!("TLS Error: {:?}", e)))
            })
            .filter(|i| futures_util::future::ready(i.is_ok())) // Need to filter out errors as they will stop server to accept connections
            .boxed();

        let service = make_service_fn(
            move |conn: &tokio_rustls::server::TlsStream<tokio::net::TcpStream>| {
                let (stream, _) = conn.get_ref();
                let addr = stream.peer_addr().unwrap();
                async move { Ok::<_, hyper::Error>(service_fn(move |req| router::route(req, addr))) }
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

impl Default for ApiServer {
    fn default() -> Self {
        Self::new()
    }
}

fn error(err: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
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

/*
-- Gracefully shut down the server --

Tutorial: https://hyper.rs/guides/server/graceful-shutdown/

*/
use std::net::SocketAddr;

// @NEW
use std::task::{Poll, Context};
use std::pin::Pin;
use std::future::Future;

use hyper::{Body, Request, Response};  // @CHANGED: Removed Server in favor for tonic::transport::Server.
use hyper::{Method, StatusCode};

use futures::TryStreamExt as _;

// @NEW
use tonic::transport::{Server, NamedService};
use tonic::body::BoxBody;
use tower::Service;

// @NEW
#[derive(Debug, Copy, Clone, Send)]
struct CustomService {}


impl NamedService for CustomService {
    const NAME: &'static str = "My Service";
}

// @NEW: https://docs.rs/tonic/0.3.0/tonic/transport/server/struct.Server.html
// Needs to implement the types Response, Error and Future, and the functions poll_ready and call.
impl Service<Request<Body>> for CustomService
{
    type Response = Response<Body>;
    type Error    = hyper::Error;
    // Pin guarantees the heap allocated pointer (a.k.a. Box type) is fixed to its memory address,
    // i.e. it cannot be moved. So this Future is a heap allocated pointer with a fixed memory
    // address pointing to a future that contains a Result of either a Response or an Error.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let response = service(request);
        Box::pin(response)
    }
}


async fn shutdown_signal() {
    // Wait for the CTRL+C signal.
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}


async fn reverse_response(request: Request<Body>) -> Result<Body, hyper::Error> {
    // Await the full body to be concatenated into a single `Bytes`...
    let full_body = hyper::body::to_bytes(request.into_body()).await?;

    // Iterate the full body in reverse order and collect into a new Vec.
    let reversed = full_body.iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();

    Ok(reversed.into())
}


fn uppercase_response(request: Request<Body>) -> Body {
    let mapping = request
        .into_body()
        .map_ok(|chunk| {
            chunk.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Vec<u8>>()
        });

    // Use `Body::wrap_stream` to convert it to a `Body`...
    Body::wrap_stream(mapping)
}


async fn service(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = request.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            *response.body_mut() = uppercase_response(request);
        },
        (&Method::POST, "/echo/reverse") => {
            *response.body_mut() = reverse_response(request).await?;
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}


#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));


    // @CHANGED: https://docs.rs/tonic/0.3.0/tonic/transport/server/struct.Server.html
    let server = Server::builder().       // Create a new server builder that can configure a Server.
        add_service(CustomService{}).     // Returns a Router that routes to the service.
        serve(address).await;             // Serves the Server.


    // And now add a graceful shutdown signal and wait.
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
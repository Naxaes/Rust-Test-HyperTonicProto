/*
-- Simple "Hello, world!" server --
Any request to 127.0.0.1:3000 will respond with "Hello, World".

Tutorial: https://hyper.rs/guides/server/hello-world/
Service: https://docs.rs/hyper/0.13.9/hyper/service/trait.Service.html

A Service lets you define how to respond to incoming requests. It's a function of a Request.
It immediately returns a Future representing the eventual completion of processing the request.
The actual request processing may happen at any time in the future, on any thread or executor.
The processing may depend on calling other services. At some point in the future, the processing
will complete, and the Future will resolve to a response or error.

In this example, we don’t have any state to carry around, so we really just need a simple async
function.
*/
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};


async fn service(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new("Hello, World".into()))
}


#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `service` function.
    let make_service = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(service))
    });

    let server = Server::bind(&address).serve(make_service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
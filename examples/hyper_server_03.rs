/*
-- New route that reacts on the stream of data POSTed --
A request to 127.0.0.1:3000/echo/uppercase will respond with the same data but uppercase.

Tutorial: https://hyper.rs/guides/server/echo/

A Body implements the Stream trait from futures, producing a bunch of Bytes, as data comes in.
Bytes is just a convenient type from hyper that represents a bunch of bytes. It can be easily
converted into other typical containers of bytes.
*/
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use hyper::{Method, StatusCode};

use futures::TryStreamExt as _; // @NEW

// @NEW
fn uppercase_response(request: Request<Body>) -> Body {
    let mapping = request
        .into_body()
        .map_ok(|chunk| {
            chunk.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Vec<u8>>()
        });

    // Use `Body::wrap_stream` to convert it to a `Body`.
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
        // @NEW
        (&Method::POST, "/echo/uppercase") => {
            *response.body_mut() = uppercase_response(request);
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
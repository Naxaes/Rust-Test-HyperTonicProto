use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use hyper::{Method, StatusCode};  // @NEW

/*
-- Simple server with routing --
GET requests to 127.0.0.1:3000 will respond with "Try POSTing data to /echo".
POST requests to 127.0.0.1:3000/echo will respond with the data sent (try `curl -d 'Hello' 127.0.0.1:3000/echo`).
Other requests will respond with status code 404 Not Found.

Tutorial: https://hyper.rs/guides/server/echo/
cUrl: https://ec.haxx.se/http/http-post

*/


// @CHANGED
async fn service(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = request.into_body();
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
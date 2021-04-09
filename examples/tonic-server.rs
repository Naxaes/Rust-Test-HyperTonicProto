use std::{
    collections::HashMap,
    hash::{Hasher, Hash},
    task::{Context, Poll},
    pin::Pin,
    sync::Arc,
    time::Instant,
};

use futures_util::StreamExt;
use futures_core::Stream;

use hyper::{Body, Request as HyperRequest, Response as HyperResponse};

use tower::Service;

use tokio::sync::mpsc;

use tonic::{Request, Response, Status, metadata::MetadataValue};
use tonic::body::BoxBody;
use tonic::transport::{Identity, Server, NamedService, ServerTlsConfig};



// Generated from .proto file.
pub mod route_guide {tonic::include_proto!("route_guide"); /* The string must match the proto package name */}
use route_guide::route_guide_server::{RouteGuide, RouteGuideServer};
use route_guide::{Feature, Point, Rectangle, RouteNote, RouteSummary};

#[path = "../src/data.rs"] mod data;


impl Hash for Point {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}

fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let top = cmp::max(lo.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    point.longitude >= left
        && point.longitude <= right
        && point.latitude >= bottom
        && point.latitude <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
fn get_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longitude as f64 / CORD_FACTOR;
    let lng2 = p2.longitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}


#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}


#[tonic::async_trait]  // Adds support for async functions in traits.
impl RouteGuide for RouteGuideService {
    type ListFeaturesStream = mpsc::Receiver<Result<Feature, Status>>;
    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + Sync + 'static>>;

    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }

        Ok(Response::new(Feature::default()))
    }

    async fn list_features(&self, request: Request<Rectangle>)
        -> Result<Response<Self::ListFeaturesStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(rx))
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        let mut stream = request.into_inner();

        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = Instant::now();

        while let Some(point) = stream.next().await {
            let point = point?;
            summary.point_count += 1;

            for feature in &self.features[..] {
                if feature.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }

            if let Some(ref last_point) = last_point {
                summary.distance += get_distance(last_point, &point);
            }

            last_point = Some(point);
        }

        summary.elapsed_time = now.elapsed().as_secs() as i32;

        Ok(Response::new(summary))
    }

    async fn route_chat(
        &self,
        request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        let mut notes = HashMap::new();
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;

                let location = note.location.clone().unwrap();

                let location_notes = notes.entry(location).or_insert(vec![]);
                location_notes.push(note);

                for note in location_notes {
                    yield note.clone();
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RouteChatStream))
    }
}

fn check_authentication(request: Request<()>) -> Result<Request<()>, Status> {
    let token = "1234";
    let token = MetadataValue::from_str(&format!("Bearer {}", token)).unwrap();

    match request.metadata().get("authorization") {
        Some(t) if token == t => Ok(request),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}


#[derive(Debug, Clone)]
struct InterceptedService<S> {
    inner: S,
}

impl<S> Service<HyperRequest<Body>> for InterceptedService<S>
    where
        S: Service<HyperRequest<Body>, Response = HyperResponse<BoxBody>>
        + NamedService
        + Clone
        + Send
        + 'static,
        S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Whether the service is ready to be called or not.
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: HyperRequest<Body>) -> Self::Future {
        let mut svc = self.inner.clone();

        Box::pin(async move {
            // Do async work here....
            println!("Work is being done...");
            svc.call(req).await
        })
    }
}

impl<S: NamedService> NamedService for InterceptedService<S> {
    const NAME: &'static str = S::NAME;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS.
    let cert = tokio::fs::read("data/tls/server.pem").await?;
    let key  = tokio::fs::read("data/tls/server.key").await?;
    let identity = Identity::from_pem(cert, key);
    let tls_config = ServerTlsConfig::new().identity(identity);

    // Load-balancing.
    let (tx, mut rx) = mpsc::unbounded_channel();
    let addresses = ["[::1]:50051", "[::1]:50052"]
        .iter()
        .map(|endpoint| endpoint.parse().unwrap());

    // Load database.
    let database = data::load();

    // Create servers.
    for address in addresses {
        let service = InterceptedService {
            inner: RouteGuideServer::with_interceptor(
                RouteGuideService { features: Arc::new(database.clone()) },
                check_authentication
            )
        };

        let serve = Server::builder().
            tls_config(tls_config.clone())?.  // Returns a Server with TLS configuration.
            add_service(service).             // Returns a Router that routes to the service.
            serve(address);                   // Serves the Server (it's async so it's not called until await).

        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });
    }

    rx.recv().await;

    Ok(())
}

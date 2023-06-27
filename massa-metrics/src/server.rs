use std::net::SocketAddr;

use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use massa_channel::MassaChannel;
use prometheus::{Encoder, TextEncoder};
use tokio::runtime::Runtime;
use tracing::{error, info};

use crate::MetricsStopper;

#[allow(dead_code)]
pub(crate) fn bind_metrics(addr: SocketAddr) -> MetricsStopper {
    let (tx, rx) = MassaChannel::new("metrics_stopper".to_string(), Some(1));
    let handle = std::thread::spawn(move || {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let server = hyper::Server::bind(&addr).serve(make_service_fn(|_| async {
                Ok::<_, hyper::Error>(service_fn(serve_req))
            }));

            let graceful_server = server.with_graceful_shutdown(async {
                rx.recv().unwrap();
            });
            info!("METRICS | listening on http://{}", addr);
            if let Err(e) = graceful_server.await {
                error!("metrics server error: {}", e);
            }
            info!("METRICS | server stopped");
        });
    });
    MetricsStopper {
        stopper: Some(tx),
        stop_handle: Some(handle),
    }
}

#[allow(dead_code)]
async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.uri().path() != "/metrics" {
        // return hyper error
        Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap())
    } else {
        let encoder = TextEncoder::new();
        let mut buffer = vec![];
        encoder
            .encode(&prometheus::gather(), &mut buffer)
            .expect("Failed to encode metrics");

        let response = Response::builder()
            .status(200)
            .header(CONTENT_TYPE, encoder.format_type())
            .body(Body::from(buffer))
            .unwrap();

        Ok(response)
    }
}
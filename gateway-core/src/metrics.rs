use prometheus::{Encoder, IntCounter, Opts, Registry, TextEncoder};
use std::sync::Arc;

#[derive(Clone)]
pub struct Metrics {
    registry: Registry,
    requests_total: IntCounter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        let requests_total = IntCounter::new("http_requests_total", "Total requests").unwrap();
        
        registry.register(Box::new(requests_total.clone())).unwrap();

        Metrics {
            registry,
            requests_total,
        }
    }

    pub fn inc_requests(&self) {
        self.requests_total.inc();
    }
}

pub fn setup_metrics() -> Arc<Metrics> {
    Arc::new(Metrics::new())
}

pub async fn metrics_handler(metrics: web::Data<Arc<Metrics>>) -> HttpResponse {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&metrics.registry.gather(), &mut buffer).unwrap();
    
    HttpResponse::Ok()
        .content_type(prometheus::TEXT_FORMAT)
        .body(buffer)
}
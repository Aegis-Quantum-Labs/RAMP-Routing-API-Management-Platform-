use actix_web::{dev::ServiceRequest, Error};
use rate_limiter::RateLimiterMiddleware;
use std::sync::Arc;

pub struct RateLimiterMiddlewareWrapper {
    middleware: Arc<RateLimiterMiddleware>,
}

impl RateLimiterMiddlewareWrapper {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            middleware: Arc::new(RateLimiterMiddleware::new(requests_per_second)),
        }
    }
}

impl actix_web::dev::Transform<ServiceRequest> for RateLimiterMiddlewareWrapper {
    type Response = ServiceRequest;
    type Error = Error;
    type Transform = RateLimiterServiceWrapper;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _: ()) -> Self::Future {
        std::future::ready(Ok(RateLimiterServiceWrapper {
            middleware: self.middleware.clone(),
        }))
    }
}

pub struct RateLimiterServiceWrapper {
    middleware: Arc<RateLimiterMiddleware>,
}

impl actix_web::dev::Service<ServiceRequest> for RateLimiterServiceWrapper {
    type Response = ServiceRequest;
    type Error = Error;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let key = req.connection_info().peer_addr().unwrap_or("unknown");
        match self.middleware.check_key(&key) {
            Ok(_) => std::future::ready(Ok(req)),
            Err(_) => std::future::ready(Err(actix_web::error::ErrorTooManyRequests(
                "Rate limit exceeded",
            ))),
        }
    }
}
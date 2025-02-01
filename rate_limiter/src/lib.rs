use actix_web::{dev::ServiceRequest, Error};
use governor::{clock, middleware::NoOpMiddleware, state::InMemoryState, Quota, RateLimiter};
use std::{num::NonZeroU32, sync::Arc};

pub struct RateLimiterMiddleware {
    limiter: Arc<RateLimiter<InMemoryState, clock::DefaultClock, NoOpMiddleware>>,
}

impl RateLimiterMiddleware {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
        RateLimiterMiddleware {
            limiter: Arc::new(RateLimiter::new(quota)),
        }
    }
}

impl actix_web::dev::Transform<ServiceRequest> for RateLimiterMiddleware {
    type Response = ServiceRequest;
    type Error = Error;
    type Transform = RateLimiterService;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _: ()) -> Self::Future {
        std::future::ready(Ok(RateLimiterService {
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct RateLimiterService {
    limiter: Arc<RateLimiter<InMemoryState, clock::DefaultClock, NoOpMiddleware>>,
}

impl actix_web::dev::Service<ServiceRequest> for RateLimiterService {
    type Response = ServiceRequest;
    type Error = Error;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let key = req.connection_info().peer_addr().unwrap_or("unknown");
        match self.limiter.check_key(&key) {
            Ok(_) => std::future::ready(Ok(req)),
            Err(_) => std::future::ready(Err(actix_web::error::ErrorTooManyRequests(
                "Rate limit exceeded",
            ))),
        }
    }
}
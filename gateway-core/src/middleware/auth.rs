use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use auth::jwt::JwtValidator;
use std::future::{ready, Ready};

pub struct JwtMiddleware {
    validator: JwtValidator,
}

impl JwtMiddleware {
    pub fn new(secret: &str) -> Self {
        Self {
            validator: JwtValidator::new(secret),
        }
    }
}

impl actix_web::dev::Transform<ServiceRequest> for JwtMiddleware {
    type Response = ServiceRequest;
    type Error = Error;
    type Transform = JwtMiddlewareService;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _: ()) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            validator: self.validator.clone(),
        }))
    }
}

pub struct JwtMiddlewareService {
    validator: JwtValidator,
}

impl actix_web::dev::Service<ServiceRequest> for JwtMiddlewareService {
    type Response = ServiceRequest;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let token = auth_header
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        match token {
            Some(t) => match self.validator.validate_token(t) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    ready(Ok(req))
                }
                Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e))),
            },
            None => ready(Err(actix_web::error::ErrorUnauthorized("Missing token"))),
        }
    }
}
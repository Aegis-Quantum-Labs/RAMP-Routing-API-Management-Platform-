use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtMiddleware {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtMiddleware {
    pub fn new(secret: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
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
            decoding_key: self.decoding_key.clone(),
            validation: self.validation.clone(),
        }))
    }
}

pub struct JwtMiddlewareService {
    decoding_key: DecodingKey,
    validation: Validation,
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
        let token = auth_header.and_then(|v| v.to_str().ok()).and_then(|s| s.strip_prefix("Bearer "));

        match token {
            Some(t) => match decode::<Claims>(t, &self.decoding_key, &self.validation) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    ready(Ok(req))
                }
                Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e))),
            },
            None => ready(Err(actix_web::error::ErrorUnauthorized("Missing token"))),
        }
    }
}
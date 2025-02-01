use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::Error;
use futures::future::{ok, Ready};

pub struct SecurityHeaders;

impl<S> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = SecurityHeadersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = S::Future;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                header::CONTENT_SECURITY_POLICY,
                header::HeaderValue::from_static("default-src 'self'"),
            );
            res.headers_mut().insert(
                header::X_CONTENT_TYPE_OPTIONS,
                header::HeaderValue::from_static("nosniff"),
            );
            Ok(res)
        })
    }
}
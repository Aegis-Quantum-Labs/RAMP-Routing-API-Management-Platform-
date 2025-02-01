use actix_web::{web, App, HttpServer};
use auth::jwt::JwtValidator;
use config::routes::load_routes;
use rate_limiter::TokenBucketRateLimiter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_routes("config.yaml").expect("Failed to load configuration");
    let jwt_validator = JwtValidator::new(&config.jwt_secret);
    let rate_limiter = TokenBucketRateLimiter::new(config.default_rate_limit);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jwt_validator.clone()))
            .app_data(web::Data::new(rate_limiter.clone()))
            .configure(|cfg| {
                load_routes(cfg, &config.routes);
            })
            .wrap(tracing::TracingMiddleware::default())
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(&config.listen_address)?
    .run()
    .await
}
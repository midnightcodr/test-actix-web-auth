use actix_web::dev::ServiceRequest;
use actix_web::{middleware, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;

async fn auth(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    let expected = std::env::var("BEARER").expect("Please define BEARER in .env");
    if token == expected {
        Ok(req)
    } else {
        return Err(actix_web::error::ErrorUnauthorized("Oops not allowed"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(HttpAuthentication::bearer(auth))
            .service(web::resource("/").to(|| async { "Test" }))
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}

mod handler;
mod logger;
mod service;
mod repository;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();
    HttpServer::new(move || App::new().configure(handler::configure))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

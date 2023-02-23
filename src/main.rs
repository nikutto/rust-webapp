mod handler;
mod logger;
mod repository;
mod service;
mod registry;

use std::sync::Arc;

use actix_web::{App, HttpServer, web::ServiceConfig};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger()?;
    let registry = Arc::new(registry::Registry::new().await);
    HttpServer::new(move || {
        App::new()
            .configure(|cfg : &mut ServiceConfig| registry.configure(cfg))
            .configure(handler::configure)
    }).bind("0.0.0.0:8080")?
        .run()
        .await
}

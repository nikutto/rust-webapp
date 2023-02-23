mod handler;
mod logger;
mod repository;
mod service;
use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use repository::hello_repository::HelloRepository;
use service::hello_service::HelloService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();
    log::info!("Creating app_datas...");
    let hello_repository = Arc::new(HelloRepository::new().await);
    let hello_service = Arc::new(HelloService::new(hello_repository));
    log::info!("All app_data have been created!");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(hello_service.clone()))
            .configure(handler::configure)
    }).bind("0.0.0.0:8080")?
        .run()
        .await
}

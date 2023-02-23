mod handler;
mod logger;
mod repository;
mod service;
mod registry;

use std::{sync::Arc, collections::HashMap};

use actix_web::{App, HttpServer, web::ServiceConfig};
use actix_web_prometheus::PrometheusMetricsBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger()?;
    let registry = Arc::new(registry::Registry::new().await);
    let mut labels = HashMap::new();
    labels.insert("service".to_string(), "app".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/internal/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .configure(|cfg : &mut ServiceConfig| registry.configure(cfg))
            .configure(handler::configure)
    }).bind("0.0.0.0:8080")?
        .run()
        .await
}

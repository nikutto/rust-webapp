use crate::service::hello_service;

use actix_web::{get, post, HttpResponse, Responder, web::ServiceConfig};
use serde::Serialize;

#[derive(Serialize)]
struct SimpleMessage {
    message: String,
}

#[get("/api/v1/hello")]
async fn hello() -> impl Responder {
    let msg = hello_service::get_hello_msg();
    HttpResponse::Ok().json(SimpleMessage { message: msg })
}

#[post("/api/v1/hello/log-check/info")]
async fn log_check_info() -> impl Responder {
    hello_service::gen_info_log();
    HttpResponse::NoContent()
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(hello);
    cfg.service(log_check_info);
}

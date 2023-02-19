use crate::service::hello_service;

use actix_web::{get, post, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct SimpleMessage {
    message: String,
}

#[get("/api/v1/hello")]
pub async fn hello() -> impl Responder {
    let msg = hello_service::get_hello_msg();
    HttpResponse::Ok().json(SimpleMessage { message: msg })
}

#[post("/api/v1/hello/log-check/info")]
pub async fn log_check_info() -> impl Responder {
    hello_service::gen_info_log();
    HttpResponse::NoContent()
}

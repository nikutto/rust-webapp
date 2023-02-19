use crate::service::hello_service;

use actix_web::{get, HttpResponse, Responder};
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

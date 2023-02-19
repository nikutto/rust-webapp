use actix_web::{Responder, HttpResponse, get};
use serde::Serialize;

#[derive(Serialize)]
struct SimpleMessage {
    message: String,
}

#[get("/api/v1/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(
        SimpleMessage { message: "hello world".to_string() }
    )
}

use std::sync::Arc;

use crate::service::hello_service::HelloService;

use actix_web::{get, post, HttpResponse, Responder, web::{Data, ServiceConfig}};
use serde::Serialize;

#[derive(Serialize)]
struct SimpleMessage {
    message: String,
}

#[get("/api/v1/hello/1")]
async fn hello1(service: Data<Arc<HelloService>>) -> impl Responder {
    let msg = service.get_hello_msg().await;
    HttpResponse::Ok().json(SimpleMessage { message: msg })
}

#[get("/api/v1/hello/2")]
async fn hello2(service: Data<Arc<HelloService>>) -> impl Responder {
    let msg = service.get_hello_msg2().await;
    match msg {
        Ok(msg) => HttpResponse::Ok().json(SimpleMessage { message: msg }),
        Err(e) => {
            log::error!("Error occurred during processing /api/v1/hello/2. {:?}", e);
            HttpResponse::InternalServerError().json(SimpleMessage {
                message: "Internal Server Error".to_string(),
            })
        }
    }
}

#[post("/api/v1/hello/log-check/info")]
async fn log_check_info(service: Data<Arc<HelloService>>) -> impl Responder {
    service.gen_info_log().await;
    HttpResponse::NoContent()
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(hello1);
    cfg.service(hello2);
    cfg.service(log_check_info);
}

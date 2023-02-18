use serde::Serialize;
use actix_web::{App, HttpServer, Responder, HttpResponse};
use actix_web::web::get;

#[derive(Serialize)]
struct SimpleMessage {
    message: String,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().json(
        SimpleMessage { message: "hello world".to_string() }
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/api/hello", get().to(hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

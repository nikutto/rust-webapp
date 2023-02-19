mod hello_handler;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.configure(hello_handler::configure);
}

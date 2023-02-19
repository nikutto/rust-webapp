mod hello;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(hello::hello);
}

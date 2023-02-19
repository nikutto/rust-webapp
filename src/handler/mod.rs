mod hello_handler;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg
        .service(hello_handler::hello)
        .service(hello_handler::log_check_info);
}

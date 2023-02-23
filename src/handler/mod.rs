mod hello_handler;

use actix_web::web::ServiceConfig;

pub fn configure<'a>(cfg: &'a mut ServiceConfig) {
    log::info!("Configuring handlers...");
    hello_handler::configure(cfg);
    log::info!("All handlers have been configured!");
}

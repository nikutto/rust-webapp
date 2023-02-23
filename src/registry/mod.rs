use std::sync::Arc;

use crate::service::hello_service::HelloService;
use crate::repository::hello_repository::HelloRepository;
use actix_web::web::{ServiceConfig, Data};

pub struct Registry {
    pub hello_service: Arc<HelloService>,
    pub hello_repository: Arc<HelloRepository>,
}

impl Registry {

    pub async fn new() -> Self {
        log::info!("Creating app_datas...");
        let hello_repository = Arc::new(HelloRepository::new().await);
        let hello_service = Arc::new(HelloService::new(hello_repository.clone()));
        log::info!("All app_data have been created!");
        Self {
            hello_service: hello_service,
            hello_repository: hello_repository.clone(),
        }
    }

    pub fn configure(&self, cfg: &mut ServiceConfig) {
        cfg.app_data(Data::new(self.hello_service.clone()));
    }
}
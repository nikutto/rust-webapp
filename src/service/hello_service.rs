use std::sync::Arc;

use crate::repository::hello_repository::HelloRepository;

#[derive(Clone)]
pub struct HelloService {
    hello_repository: Arc<HelloRepository>
}

impl HelloService {

    pub fn new(hello_repository: Arc<HelloRepository>) -> Self {
        HelloService { hello_repository: hello_repository }
    }

    pub async fn get_hello_msg(&self) -> String {
        "Hello world!".to_string()
    }
    
    pub async fn get_hello_msg2(&self) -> Result<String, sqlx::Error> {
        self.hello_repository.get_hello_message().await
    }
    
    pub async fn gen_info_log(&self) {
        log::info!("Test logging info log.")
    }
}

use crate::repository::hello_repository;

pub async fn get_hello_msg() -> String {
    "Hello world!".to_string()
}

pub async fn get_hello_msg2() -> Result<String, sqlx::Error> {
    hello_repository::get_hello_message().await
}

pub async fn gen_info_log() {
    log::info!("Test logging info log.")
}

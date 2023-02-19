pub async fn get_hello_msg() -> String {
    "Hello world!".to_string()
}

pub async fn test_hello_repository() {
    let result = crate::repository::test_hello_repository().await;
    match result {
        Ok(_) => {}
        Err(e) => {
            log::error!("{:?}", e);
        }
    }
}

pub async fn gen_info_log() {
    log::info!("Test logging info log.")
}
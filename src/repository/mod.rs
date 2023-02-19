use sqlx::mysql::MySqlPoolOptions;

pub async fn test_hello_repository() -> Result<(), sqlx::Error> {
    log::info!("Start");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://user:password@127.0.0.1/db").await?;
    log::info!("Obtained pool");
    let row: (i64, String) = sqlx::query_as("SELECT id, msg FROM hello_message WHERE id = 0")
        .fetch_one(&pool).await?;
    log::info!("{:?}", row);
    Ok(())
}
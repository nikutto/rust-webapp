use sqlx::{mysql::{MySqlPoolOptions, MySqlConnectOptions}, Pool, MySql};

#[derive(Clone)]
pub struct HelloRepository {
    pool: Pool<MySql>,
}

impl HelloRepository {
    const GET_QUERY: &str = "SELECT id, msg FROM hello_message WHERE id = ?";

    pub async fn new() -> Self {
        log::info!("Creating MySQL connection pool...");
        let connect_option = MySqlConnectOptions::new()
            .host("127.0.0.1")
            .username("user")
            .password("password")
            .database("db");
        let pool = MySqlPoolOptions::new()
                .connect_with(connect_option)
                .await
                .unwrap();
        log::info!("Created MySQL connection pool!");
        HelloRepository { pool: pool }
    }

    pub async fn get_hello_message(&self) -> Result<String, sqlx::Error> {
        let (_, msg): (i64, String) = sqlx::query_as(Self::GET_QUERY).bind(0).fetch_one(&self.pool).await?;
        return Ok(msg);
    }
}

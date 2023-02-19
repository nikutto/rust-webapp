use sqlx::mysql::MySqlPoolOptions;

const GET_QUERY: &str = "SELECT id, msg FROM hello_message WHERE id = ?";

pub async fn get_hello_message() -> Result<String, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://user:password@127.0.0.1/db")
        .await?;
    let (_, msg): (i64, String) = sqlx::query_as(GET_QUERY).bind(0).fetch_one(&pool).await?;
    return Ok(msg);
}

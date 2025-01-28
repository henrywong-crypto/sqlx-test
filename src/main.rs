use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}

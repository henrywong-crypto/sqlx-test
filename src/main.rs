use sqlx::sqlite::SqlitePool;
use std::env;
use time::OffsetDateTime;

async fn add_user(pool: &SqlitePool) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    let now = OffsetDateTime::now_utc();

    let id = sqlx::query!(
        //        r#"
        //INSERT INTO users ( created_at )
        //VALUES ( ?1 )
        //        "#,
        r#"
INSERT INTO users DEFAULT VALUES
        "#
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    add_user(&pool).await?;
    //let row: (i64,) = sqlx::query_as("SELECT $1")
    //    .bind(150_i64)
    //    .fetch_one(&pool)
    //    .await?;

    //assert_eq!(row.0, 150);

    Ok(())
}

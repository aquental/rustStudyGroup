use sqlx::Row;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let row = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;
    let sum: i32 = row.get("sum"); // 0-indexed
    println!("1 + 1 = {}", sum);
    Ok(())
}

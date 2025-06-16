use sqlx::Row;
use std::error::Error;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<Book, sqlx::Error> {
    let query = "INSERT INTO books (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    let book = Book {
        title: book.title.clone(),
        author: book.author.clone(),
        isbn: book.isbn.clone(),
    };
    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let row = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;
    let sum: i32 = row.get("sum");
    println!("1 + 1 = {}", sum);

    // Demonstrate using the Book struct and create function
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        isbn: "978-1593278281".to_string(),
    };
    
    match create(&book, &pool).await {
        Ok(created_book) => println!("Created book: {} by {}", created_book.title, created_book.author),
        Err(e) => println!("Failed to create book: {}", e),
    }
    
    Ok(())
}

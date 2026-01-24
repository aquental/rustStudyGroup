# SQLx

Here is a concise introduction to **SQLx**, one of the most popular async SQL libraries in the Rust ecosystem.

**SQLx** is an **async**, **pure-Rust** SQL toolkit that emphasizes **type-safe**, **compile-time checked queries** without requiring a domain-specific language (DSL) or heavy code generation.

Its most famous feature is **compile-time query verification**: SQLx can connect to your database at compile time (when the `offline` mode isn't used) to verify that:

- Your SQL queries are syntactically correct
- The result columns match the Rust type you're mapping to
- The parameters you're binding have compatible types

This catches many classes of errors that would otherwise only appear at runtime.

## Main Features

- Supports **PostgreSQL**, **MySQL** / **MariaDB**, **SQLite**, and **Microsoft SQL Server** (MSSQL)
- Fully **async** (works with Tokio, async-std via feature flags, and experimental support for smol / async-global-executor)
- No DSL → you write **plain SQL** (with optional macros for extra safety)
- Very lightweight runtime & compile-time dependencies
- Built-in support for connection pooling
- Query macros: `query!()`, `query_as!()`, `query_scalar!()`
- `.fetch()`, `.fetch_one()`, `.fetch_optional()`, `.execute()`, etc.
- Excellent support for transactions, prepared statements, and raw SQL when needed
- Offline mode (using cached query analysis) for faster builds / CI

## Key Links (as of 2026)

- **Official Repository** (GitHub – main source of truth)  
  https://github.com/launchbadge/sqlx

- **API Documentation** (latest stable version)  
  https://docs.rs/sqlx

- **Crates.io page** (for version history & dependencies)  
  https://crates.io/crates/sqlx

- **Book / Guide** (highly recommended reading)  
  The README itself serves as the main guide, but many excellent community tutorials exist. Start here:  
  https://github.com/launchbadge/sqlx/blob/main/README.md

## Quick "Hello World" Example (Tokio + PostgreSQL)

```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use sqlx::{postgres::PgPoolOptions, FromRow};

#[derive(FromRow)]
struct User {
    id: i32,
    username: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/database")
        .await?;

    // Compile-time checked query
    let users = sqlx::query_as::<_, User>("SELECT id, username FROM users WHERE id > $1")
        .bind(10)
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{} → {}", user.id, user.username);
    }

    Ok(())
}
```

## Common Macro Variants

```rust
// Returns Vec<(String, i64)>
let rows = sqlx::query!("SELECT name, count FROM stats")
    .fetch_all(&pool).await?;

// Maps directly to a struct (needs #[derive(sqlx::FromRow)])
let users = sqlx::query_as!(User, "SELECT id, username FROM users")
    .fetch_all(&pool).await?;

// Single scalar value
let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM orders")
    .fetch_one(&pool).await?;
```

## When to choose SQLx?

Choose SQLx if you want:

- Maximum **type safety** with plain SQL
- Zero-cost abstractions where possible
- Good performance (very competitive with other async Rust DB libraries)
- Flexibility (raw SQL + macros)

Popular alternatives for comparison:

- **Diesel** → more DSL-heavy, sync-first (though async support exists)
- **SeaORM** → full ORM with entity relations & migrations
- **tokio-postgres** / **mysql_async** → lower-level, no query checking

SQLx strikes an excellent balance for most production web services and CLI tools in Rust.

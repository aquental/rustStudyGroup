mod handlers;
mod models;
mod errors;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::sqlite::{SqlitePoolOptions};
use std::env;
use std::net::SocketAddr;
use handlers::*;
use models::{AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new().connect(&db_url).await?;

    let app_state = AppState { pool: pool.clone() };

    let app = Router::new()
        .route("/courses", post(create_course))
        .route("/courses", get(list_courses))
        .route(
            "/courses/{id}",
            get(get_course).put(update_course).delete(delete_course),
        )
        .with_state(app_state);

    println!("Pachadata API at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(
        SocketAddr::from(([0, 0, 0, 0], 3000))
    ).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

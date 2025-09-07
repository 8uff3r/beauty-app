mod error;
mod router;
use crate::router::*;
use std::{env, sync::Arc};

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqlitePool, SqlitePoolOptions},
};

pub struct AppState {
    db: SqlitePool,
}
pub type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL environment variable not set")
        .unwrap();
    tracing_subscriber::fmt::init();
    let pool = setup_database(&database_url).await?;
    let state = Arc::new(AppState { db: pool });

    let app = Router::<SharedState>::new()
        .route("/", get(root))
        .merge(users_router())
        .merge(stores_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
async fn root() -> &'static str {
    "Hello, World!"
}

async fn setup_database(db_url: &str) -> Result<SqlitePool> {
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        sqlx::Sqlite::create_database(db_url).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stores (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            address TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

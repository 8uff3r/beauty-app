mod error;
use crate::error::*;
use std::{env, sync::Arc};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqlitePool, SqlitePoolOptions},
};

struct AppState {
    db: SqlitePool,
}
type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL environment variable not set")
        .unwrap();
    tracing_subscriber::fmt::init();
    let pool = setup_database(&database_url).await?;
    let state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let AppState { db } = &*state;
    let user = sqlx::query!(
        r#"INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email"#,
        payload.name,
        payload.email
    )
    .fetch_one(db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(User {
            id: user.id,
            user: user.name,
            email: user.email,
        }),
    ))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: i64,
    user: String,
    email: String,
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

    Ok(pool)
}

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::{AppState, SharedState, error::AppError};

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct User {
    id: i64,
    user: String,
    email: String,
}
pub fn users_router() -> Router<SharedState> {
    Router::new().route("/users", post(create_user))
}
pub async fn create_user(
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

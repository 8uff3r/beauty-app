use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};

use crate::{AppState, SharedState, error::AppError};

#[derive(Deserialize)]
pub struct CreateStore {
    name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    id: i64,
    name: String,
    address: String,
}

pub fn stores_router() -> Router<SharedState> {
    Router::new().route("/stores", get(get_stores))
}
pub async fn get_stores(
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<Vec<Store>>), AppError> {
    let AppState { db } = &*state;
    let stores = sqlx::query!(
        r#"
        SELECT id, name, address FROM stores
    "#
    )
    .fetch_all(db)
    .await?;
    println!("{stores:?}");

    let stores: Vec<Store> = stores
        .into_iter()
        .map(|s| Store {
            id: s.id,
            name: s.name,
            address: s.address,
        })
        .collect();

    Ok((StatusCode::OK, Json(stores)))
}

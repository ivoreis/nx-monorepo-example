use axum::{extract::Extension, Json};
use serde_json::json;
use sqlx::Pool;
use std::sync::Arc;

use crate::fake;

pub async fn generate_fake_data(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
) -> Json<serde_json::Value> {
    let _ = fake::populate_fake_data(pool.clone()).await;

    Json(json!({"status": "Data Generated"}))
}

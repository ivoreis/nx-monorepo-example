use axum::{extract::Extension, extract::Path, http::StatusCode, Json};
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::db;
use crate::models;

pub async fn get_orders(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
) -> Result<Json<Vec<models::Order>>, (StatusCode, String)> {
    let orders = db::get_orders(&*pool).await.map_err(|e| {
        eprintln!("Error querying orders: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve orders".to_string(),
        )
    })?;

    Ok(Json(orders))
}

pub async fn get_order_by_id(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<models::Order>, axum::http::StatusCode> {
    let order = db::get_order(&*pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(axum::Json(order))
}

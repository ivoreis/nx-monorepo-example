use axum::{extract::Extension, extract::Path, http::StatusCode, Json};
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db, models};

pub async fn get_tasks(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
) -> Result<Json<Vec<models::Task>>, (StatusCode, String)> {
    let tasks = db::get_tasks(&*pool).await.map_err(|e| {
        eprintln!("Error querying tasks: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve tasks".to_string(),
        )
    })?;

    Ok(Json(tasks))
}

pub async fn get_task_by_id(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<models::Task>, axum::http::StatusCode> {
    let task = db::get_task(&*pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(axum::Json(task))
}

pub async fn get_tasks_by_order_id(
    Extension(pool): Extension<Arc<Pool<sqlx::Postgres>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<models::Task>>, (StatusCode, String)> {
    let tasks = db::get_tasks_by_order(&*pool, id).await.map_err(|e| {
        eprintln!("Error querying tasks: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve tasks".to_string(),
        )
    })?;

    Ok(Json(tasks))
}

use axum::{routing::get, Router};
use sqlx::Pool;
use std::sync::Arc;

mod faker;
mod orders;
mod tasks;

pub fn build_router(pool: Arc<Pool<sqlx::Postgres>>) -> Router {
    Router::new()
        .route("/orders", get(orders::get_orders))
        .route("/orders/:id", get(orders::get_order_by_id))
        .route("/orders/:id/tasks", get(tasks::get_tasks_by_order_id))
        .route("/tasks", get(tasks::get_tasks))
        .route("/tasks/:id", get(tasks::get_task_by_id))
        .route("/generate", get(faker::generate_fake_data))
        .layer(axum::extract::Extension(pool))
}

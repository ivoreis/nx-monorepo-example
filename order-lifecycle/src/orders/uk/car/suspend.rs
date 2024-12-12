use crate::models::{Order, OrderError, Task};
use sqlx::Pool;

pub async fn suspend_task(
    _pool: &Pool<sqlx::Postgres>,
    task: Task,
    order: Order,
) -> Result<Order, OrderError> {
    println!("Handling Suspension Task ID: {}", task.id);
    Ok(order)
}
use crate::models::{Order, OrderError, Task};
use sqlx::Pool;

pub async fn update_task(
    _pool: &Pool<sqlx::Postgres>,
    task: Task,
    order: Order,
) -> Result<Order, OrderError> {
    println!("Handling Update Task ID: {}", task.id);
    Ok(order)
}

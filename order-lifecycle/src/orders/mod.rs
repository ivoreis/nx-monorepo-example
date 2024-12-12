use crate::models::{Order, OrderError, Task};
use sqlx::Pool;

pub mod uk;

#[async_trait::async_trait]
pub trait OrderHandler: Send + Sync {
    async fn handle_task(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
    ) -> Result<Order, OrderError>;
    async fn issue(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn activate(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn suspend(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn update(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn cancel(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn terminate(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
    async fn audit(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError>;
}

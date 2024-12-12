mod activate;
mod audit;
mod cancel;
mod issue;
mod suspend;
mod terminate;
mod update;

use activate::activate_task;
use audit::audit_task;
use cancel::cancel_task;
use issue::issue_task;
use sqlx::Pool;
use suspend::suspend_task;
use terminate::terminate_task;
use update::update_task;

use crate::{
    db::get_order,
    models::{ActionType, Order, OrderError, Task},
    orders::OrderHandler,
};

pub struct CarOrder;

#[async_trait::async_trait]
impl OrderHandler for CarOrder {
    async fn handle_task(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
    ) -> Result<Order, OrderError> {
        let order = get_order(pool, task.order_id).await?;
        let task_clone = task.clone();

        let order_result = match task.action {
            ActionType::Issue => self.issue(pool, task_clone, order).await,
            ActionType::Activate => self.activate(pool, task_clone, order).await,
            ActionType::Update => self.update(pool, task_clone, order).await,
            ActionType::Suspend => self.suspend(pool, task_clone, order).await,
            ActionType::Cancel => self.cancel(pool, task_clone, order).await,
            ActionType::Terminate => self.terminate(pool, task_clone, order).await,
        };

        if let Ok(updated_order) = &order_result {
            if let Err(audit_error) = self.audit(pool, task.clone(), updated_order.clone()).await {
                eprintln!(
                    "Audit failed for task {:?} on order {:?}: {:?}",
                    task.action, updated_order.id, audit_error
                );
            }
        }

        order_result
    }

    async fn issue(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        issue_task(pool, task, order).await
    }

    async fn activate(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        activate_task(pool, task, order).await
    }

    async fn suspend(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        suspend_task(pool, task, order).await
    }

    async fn update(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        update_task(pool, task, order).await
    }

    async fn cancel(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        cancel_task(pool, task, order).await
    }

    async fn terminate(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        terminate_task(pool, task, order).await
    }

    async fn audit(
        &self,
        pool: &Pool<sqlx::Postgres>,
        task: Task,
        order: Order,
    ) -> Result<Order, OrderError> {
        audit_task(pool, task, order).await
    }
}

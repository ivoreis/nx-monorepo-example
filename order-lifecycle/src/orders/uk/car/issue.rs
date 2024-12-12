use crate::db::update_order_state;
use crate::models::{Order, OrderError, OrderState, Task};
use sqlx::Pool;

pub async fn issue_task(
    pool: &Pool<sqlx::Postgres>,
    task: Task,
    order: Order,
) -> Result<Order, OrderError> {
    println!("Handling Issue Task ID {}", task.id);

    if order.state != OrderState::Requested {
        eprintln!(
            "Cannot issue Order ID {}: Current state is {:?}",
            order.id, order.state
        );
        return Err(OrderError::InvalidStateTransition(order.state.to_string()));
    }

    let updated_order = update_order_state(pool, order.id, OrderState::Issued).await?;

    println!("Order ID {} successfully issued.", updated_order.id);

    Ok(updated_order)
}

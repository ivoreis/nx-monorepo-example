use crate::db::update_order_state;
use crate::models::{Order, OrderError, OrderState, Task};
use sqlx::Pool;

pub async fn cancel_task(
    pool: &Pool<sqlx::Postgres>,
    task: Task,
    order: Order,
) -> Result<Order, OrderError> {
    println!("Handling Cancel Task ID {}", task.id);
    let accepted_values = vec![
        OrderState::Requested,
        OrderState::Issued,
        OrderState::Active,
        OrderState::Suspended,
    ];

    if !accepted_values.contains(&order.state) {
        eprintln!(
            "Cannot cancel Order ID {}: Current state is {:?}",
            order.id, order.state
        );
        return Err(OrderError::InvalidStateTransition(order.state.to_string()));
    }

    let updated_order = update_order_state(pool, order.id, OrderState::Cancelled).await?;

    println!("Order ID {} successfully cancelled.", updated_order.id);

    Ok(updated_order)
}

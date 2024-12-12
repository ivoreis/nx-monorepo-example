use crate::db::update_order_state;
use crate::models::{CarPayload, Order, OrderError, OrderPayload, OrderState, Task};
use sqlx::Pool;

fn handle_v1(order: Order) -> Result<(), OrderError> {
    if order.state != OrderState::Issued {
        eprintln!(
            "Cannot activate Order ID {}: Current state is {:?}",
            order.id, order.state
        );
        return Err(OrderError::InvalidStateTransition(order.state.to_string()));
    }
    Ok(())
}

pub async fn activate_task(
    pool: &Pool<sqlx::Postgres>,
    task: Task,
    order: Order,
) -> Result<Order, OrderError> {
    println!("Handling Activate Task ID {}", task.id);

    let result = match order.payload {
        OrderPayload::Car {
            details: CarPayload::UK { version, .. },
        } => match version {
            1 => {
                println!(
                    "Handle car payload version 1 for country: {}",
                    order.country
                );
                let _ = handle_v1(order.clone());
                Ok(())
            }
            _ => {
                println!("Unknown version for car payload");
                Err(OrderError::InvalidVersion(version))
            }
        },
    };

    match result {
        Ok(_) => {
            let updated_order = update_order_state(pool, order.id, OrderState::Active).await?;
            println!("Order ID {} successfully activated.", updated_order.id);
            Ok(updated_order)
        }
        Err(err) => Err(err),
    }
}

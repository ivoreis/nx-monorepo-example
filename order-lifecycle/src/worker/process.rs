use crate::db::{complete_task, fail_task, generate_idempotency_key, in_progress_task};
use crate::models::{Country, Order, OrderError, Product, Task};
use crate::orders::{uk, OrderHandler};
use sqlx::Pool;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

pub async fn process_task(
    pool: &Pool<sqlx::Postgres>,
    country: Country,
    product: Product,
    task: Task,
) -> Result<Order, OrderError> {
    println!(
        "Processing {} order {}",
        str::to_uppercase(&task.action.to_string()),
        task.order_id,
    );

    let orders = {
        let mut orders = HashMap::new();
        let mut uk_orders = HashMap::new();

        uk_orders.insert(
            Product::CAR,
            Box::new(uk::car::CarOrder) as Box<dyn OrderHandler>,
        );
        orders.insert(Country::UK, uk_orders);

        orders
    };

    // Generate idempotency key
    let idempotency_key = generate_idempotency_key(task.id, &task.payload);

    // Get the appropriate handler
    let handler = get_order_handler(&orders, country, product)?;

    let _ = in_progress_task(pool, task.id).await;

    match retry_with_backoff(&task, || handler.handle_task(pool, task.clone())).await {
        Ok(order) => {
            // Mark the task as completed idempotently
            complete_task(pool, task.id, idempotency_key)
                .await
                .map_err(OrderError::DatabaseError)?;

            Ok(order)
        }
        Err(e) => {
            // Mark the task as failed idempotently
            fail_task(pool, task.id, idempotency_key)
                .await
                .map_err(OrderError::DatabaseError)?;

            Err(e)
        }
    }
}

fn get_order_handler<'a>(
    orders: &'a HashMap<Country, HashMap<Product, Box<dyn OrderHandler>>>,
    country: Country,
    product: Product,
) -> Result<&'a dyn OrderHandler, OrderError> {
    if let Some(product_handlers) = orders.get(&country) {
        if let Some(handler) = product_handlers.get(&product) {
            Ok(handler.as_ref())
        } else {
            Err(OrderError::ActionTypeNotSupported(format!(
                "No handler found for product: {:?}",
                product
            )))
        }
    } else {
        Err(OrderError::ActionTypeNotSupported(format!(
            "No handler found for country: {:?}",
            country
        )))
    }
}

/// Retry logic with exponential backoff
async fn retry_with_backoff<F, Fut, T>(task: &Task, mut operation: F) -> Result<T, OrderError>
where
    F: FnMut() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<T, OrderError>> + Send,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // TODO: extract OrderError type and decide if should be retried or not
                // some errors are not retriable
                attempt += 1;
                if attempt > task.max_retries {
                    eprintln!(
                        "Task {} for order {} failed after {} attempts: {:?}",
                        task.id, task.order_id, attempt, e
                    );
                    return Err(e);
                }

                let backoff = Duration::from_secs(2_u64.pow((attempt - 1).try_into().unwrap()));
                println!(
                    "Retrying task {} for order {} (attempt {} of {}), waiting {:?}...",
                    task.id, task.order_id, attempt, task.max_retries, backoff
                );
                sleep(backoff).await;
            }
        }
    }
}

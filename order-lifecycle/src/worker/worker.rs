use crate::db::next_task;
use crate::models::{Country, Product};
use crate::worker::process_task;
use sqlx::Pool;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn execute(
    pool: Arc<Pool<sqlx::Postgres>>,
    country: Country,
    product: Product,
) -> Result<(), String> {
    loop {
        match next_task(&pool, country, product).await {
            Ok(Some(task)) => {
                if let Err(e) = process_task(&pool, country, product, task).await {
                    eprintln!("Failed to process task: {}", e);
                }
                println!("----")
            }
            Ok(None) => {
                // println!("No pending tasks for {}:{}", country, product);
            }
            Err(e) => {
                eprintln!("Failed to fetch tasks for {}:{}, {:?}", country, product, e);
            }
        }

        sleep(Duration::from_secs(1)).await; // Wait before retrying
    }
}

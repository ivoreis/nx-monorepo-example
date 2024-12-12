use chrono::Utc;
use rand::seq::SliceRandom;
use sqlx::Pool;
use std::str::FromStr;
use std::sync::Arc;

use crate::db;
use crate::models::{
    ActionType, ActivationReason, CancellationReason, CarPayload, Country, OrderPayload,
    OrderState, Product, TaskPayload, TerminationReason,
};

pub async fn populate_fake_data(pool: Arc<Pool<sqlx::Postgres>>) -> anyhow::Result<()> {
    println!("Populating the database with fake data...");

    // Create multiple random UK::Car orders in the requested state
    create_random_orders(pool.clone(), Country::UK, Product::CAR, 10).await?;

    // Create tasks that move orders to different states
    create_tasks_for_orders(pool.clone()).await?;

    Ok(())
}

// Create multiple random orders
async fn create_random_orders(
    pool: Arc<Pool<sqlx::Postgres>>,
    country: Country,
    product: Product,
    count: usize,
) -> anyhow::Result<()> {
    println!(
        "Creating {} random {}::{} orders in the requested state...",
        count, country, product
    );

    for _ in 0..count {
        let payload = generate_order_payload(country, product);
        db::create_order(&*pool, country, product, payload).await?;
    }

    Ok(())
}

fn generate_order_payload(country: Country, product: Product) -> OrderPayload {
    match (country, product) {
        (Country::UK, Product::CAR) => OrderPayload::Car {
            details: CarPayload::UK {
                version: 1,
                model: "corsa".to_string(),
                brand: "opel".to_string(),
                color: "white".to_string(),
                price: 5000,
            },
        },
    }
}

fn generate_task_payload(action: ActionType, order_id: uuid::Uuid) -> TaskPayload {
    match action {
        ActionType::Issue => TaskPayload::Issue {
            order_id: order_id,
            effective_date: Utc::now().to_string(),
        },
        ActionType::Activate => TaskPayload::Activate {
            order_id: order_id,
            activation_date: Utc::now().to_string(),
            activation_reason: ActivationReason::Initial,
        },
        ActionType::Cancel => TaskPayload::Cancel {
            order_id: order_id,
            reason: CancellationReason::Customer,
        },
        ActionType::Suspend => TaskPayload::Suspend {
            order_id: order_id,
            reason: "not using the item".to_string(),
            suspension_period: None,
        },
        ActionType::Terminate => TaskPayload::Terminate {
            order_id: order_id,
            reason: TerminationReason::Expired,
        },
        ActionType::Update => TaskPayload::Update {
            order_id: order_id,
            updates: serde_json::json!({}),
        },
    }
}

async fn create_tasks_for_orders(pool: Arc<Pool<sqlx::Postgres>>) -> anyhow::Result<()> {
    println!("Creating tasks for previously created orders...");

    let requested_orders: Vec<uuid::Uuid> = db::get_orders_by_state(&*pool, OrderState::Requested)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect();

    let issued_orders: Vec<uuid::Uuid> = db::get_orders_by_state(&*pool, OrderState::Issued)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect();

    for order_id in requested_orders {
        let actions = ["issue", "cancel"];
        let action = actions.choose(&mut rand::thread_rng()).unwrap();

        let action_value = ActionType::from_str(action).unwrap();
        let payload = generate_task_payload(action_value, order_id); // Generate the payload for the task

        db::create_task(&pool, order_id, &action, serde_json::to_value(payload)?).await?;
    }

    for order_id in issued_orders {
        let actions = ["activate", "cancel"];
        let action = actions.choose(&mut rand::thread_rng()).unwrap();

        let action_value = ActionType::from_str(action).unwrap();
        let payload = generate_task_payload(action_value, order_id); // Generate the payload for the task

        db::create_task(&pool, order_id, &action, serde_json::to_value(payload)?).await?;
    }

    Ok(())
}

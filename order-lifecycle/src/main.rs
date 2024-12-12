use anyhow;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use std::sync::Arc;
use tokio::signal;

mod db;
mod fake;
mod models;
mod orders;
mod routes;
mod worker;

struct WorkerConfiguration {
    product: models::Product,
    num_workers: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let shared_pool = setup_database().await?;

    start_workers(shared_pool.clone()).await;
    start_web_server(shared_pool.clone()).await;

    println!("Populating fake data");
    fake::populate_fake_data(shared_pool.clone()).await?;
    println!("Fake data populated");

    wait_for_shutdown_signal().await;
    println!("Shutdown signal received. Exiting main...");
    Ok(())
}

async fn setup_database() -> anyhow::Result<Arc<Pool<sqlx::Postgres>>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Attempting to connect to: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(Arc::new(pool))
}

async fn start_workers(pool: Arc<Pool<sqlx::Postgres>>) {
    let countries = vec![models::Country::UK];
    let product_configs = vec![WorkerConfiguration {
        product: models::Product::CAR,
        num_workers: 1,
    }];

    for country in countries {
        for product_config in &product_configs {
            for worker_id in 0..product_config.num_workers {
                let pool_clone = pool.clone();
                let country = country;
                let product = product_config.product;

                tokio::spawn(async move {
                    println!("Starting {}:{} worker {}", country, product, worker_id + 1);
                    let _ = worker::execute(pool_clone, country, product).await;
                });
            }
        }
    }
}

async fn start_web_server(pool: Arc<Pool<sqlx::Postgres>>) {
    let app = routes::build_router(pool.clone());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Axum web server running at http://{}", addr);

    tokio::spawn(async move {
        if let Err(e) = axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
        {
            eprintln!("Web server error: {}", e);
        }
    });
}

async fn wait_for_shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C signal");
}

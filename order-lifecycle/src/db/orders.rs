use crate::models::{Country, Order, OrderError, OrderPayload, OrderState, Product};
use sqlx::types::chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_orders(pool: &Pool<Postgres>) -> Result<Vec<Order>, OrderError> {
    sqlx::query_as!(
        Order,
        r#"
        SELECT 
            id, 
            state as "state: OrderState", 
            country as "country: Country", 
            product as "product: Product",
            payload as "payload: OrderPayload", 
            created_at, 
            updated_at
        FROM orders
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(OrderError::DatabaseError)
}

pub async fn get_orders_by_state(
    pool: &Pool<Postgres>,
    state: OrderState,
) -> Result<Vec<Order>, OrderError> {
    sqlx::query_as!(
        Order,
        r#"
        SELECT 
            id, 
            state as "state: OrderState", 
            country as "country: Country", 
            product as "product: Product",
            payload as "payload: OrderPayload", 
            created_at, 
            updated_at
        FROM orders
        WHERE state = $1
        "#,
        state as OrderState
    )
    .fetch_all(pool)
    .await
    .map_err(OrderError::DatabaseError)
}

pub async fn get_order(pool: &Pool<Postgres>, id: Uuid) -> Result<Order, OrderError> {
    sqlx::query_as!(
        Order,
        r#"
        SELECT 
            id, 
            state as "state: OrderState", 
            country as "country: Country", 
            product as "product: Product",
            payload as "payload: OrderPayload", 
            created_at, 
            updated_at
        FROM orders
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(OrderError::DatabaseError)
}

pub async fn update_order_state(
    pool: &Pool<Postgres>,
    order_id: Uuid,
    new_state: OrderState,
) -> Result<Order, OrderError> {
    sqlx::query_as!(
        Order,
        r#"
        UPDATE orders
        SET state = $1, updated_at = $2
        WHERE id = $3
        RETURNING id,
                state as "state: OrderState",
                country as "country: Country",
                product as "product: Product",
                payload as "payload: OrderPayload",
                created_at,
                updated_at
        "#,
        new_state as OrderState,
        Utc::now(),
        order_id
    )
    .fetch_one(pool)
    .await
    .map_err(OrderError::DatabaseError)
}

pub async fn create_order(
    pool: &Pool<Postgres>,
    country: Country,
    product: Product,
    payload: OrderPayload,
) -> Result<u64, OrderError> {
    let order_id = Uuid::new_v4();

    let result = sqlx::query!(
        r#"
        INSERT INTO orders (id, country, product, state, payload, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        "#,
        order_id,
        country as Country,
        product as Product,
        OrderState::Requested as OrderState,
        payload as OrderPayload
    )
    .execute(pool)
    .await
    .map_err(OrderError::DatabaseError);

    Ok(result.unwrap().rows_affected())
}

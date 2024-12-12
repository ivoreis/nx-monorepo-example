use crate::models::{ActionType, Country, Product, Task, TaskError, TaskPayload, TaskStatus};

use sha2::{Digest, Sha256};
use sqlx::{Error, Pool, Postgres};
use std::str::FromStr;
use uuid::Uuid;

pub fn generate_idempotency_key(task_id: Uuid, payload: &TaskPayload) -> String {
    let mut hasher = Sha256::new();
    hasher.update(task_id.to_string());
    hasher.update(payload.to_string());
    format!("{:x}", hasher.finalize())
}

/// Fetches a single pending task for the given country and product.
/// Locks the task using SKIP LOCKED for concurrent processing safety.
pub async fn next_task(
    pool: &Pool<Postgres>,
    country: Country,
    product: Product,
) -> Result<Option<Task>, Error> {
    let task_query = format!(
        "SELECT t.* FROM tasks t
         JOIN orders w ON t.order_id = w.id
         WHERE t.status = 'pending'
         AND w.country = '{}' AND w.product = '{}'
         AND NOT EXISTS (
            SELECT 1
            FROM tasks t2
            WHERE t2.order_id = t.order_id
            AND t2.task_sequence < t.task_sequence
            AND (t2.status = 'pending' OR t2.status = 'failed' OR t2.status = 'in_progress')
        )
         ORDER BY t.order_id, t.task_sequence ASC
         LIMIT 1 FOR UPDATE SKIP LOCKED",
        country, product
    );

    sqlx::query_as::<_, Task>(&task_query)
        .fetch_optional(pool)
        .await
}

pub async fn complete_task(
    pool: &Pool<Postgres>,
    task_id: Uuid,
    idempotency_key: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE tasks 
        SET status = 'completed', idempotency_key = $2, updated_at = NOW()
        WHERE id = $1 
        AND (idempotency_key IS NULL OR idempotency_key != $2)
        "#,
        task_id,
        idempotency_key
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn in_progress_task(pool: &Pool<Postgres>, task_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE tasks 
        SET status = 'in_progress', updated_at = NOW()
        WHERE id = $1
        "#,
        task_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fail_task(
    pool: &Pool<Postgres>,
    task_id: Uuid,
    idempotency_key: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE tasks 
        SET status = 'failed', idempotency_key = $2, updated_at = NOW()
        WHERE id = $1 
        AND (idempotency_key IS NULL OR idempotency_key != $2)
        "#,
        task_id,
        idempotency_key
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_task(
    pool: &Pool<Postgres>,
    order_id: Uuid,
    action: &str,
    payload: serde_json::Value,
) -> Result<Uuid, sqlx::Error> {
    // Fetch the current maximum task_sequence for the order
    let max_sequence: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(MAX(task_sequence), 0)
        FROM tasks
        WHERE order_id = $1
        "#,
        order_id
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    // Increment the sequence for the new task
    let new_sequence = max_sequence + 1;

    let task_id = Uuid::new_v4();
    let payload_json = serde_json::to_value(payload).unwrap();
    let status_value = TaskStatus::from_str("pending").unwrap();
    let action_value = ActionType::from_str(action).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO tasks (id, order_id, action, status, task_sequence, payload, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        "#,
        task_id,
        order_id,
        action_value as ActionType,
        status_value as TaskStatus,
        new_sequence,
        payload_json
    )
    .execute(pool)
    .await?;

    Ok(task_id)
}

pub async fn get_task(pool: &Pool<Postgres>, id: Uuid) -> Result<Task, TaskError> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT id, 
            order_id,
            action as "action: ActionType",
            status as "status: TaskStatus",
            payload as "payload: TaskPayload",
            max_retries,
            task_sequence,
            scheduled_at,
            created_at, 
            updated_at
        FROM tasks
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(TaskError::DatabaseError)
}

pub async fn get_tasks(pool: &Pool<Postgres>) -> Result<Vec<Task>, TaskError> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT id, 
            order_id,
            action as "action: ActionType",
            status as "status: TaskStatus",
            payload as "payload: TaskPayload",
            max_retries,
            task_sequence,
            scheduled_at,
            created_at, 
            updated_at
        FROM tasks
        ORDER BY task_sequence ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(TaskError::DatabaseError)
}

pub async fn get_tasks_by_order(
    pool: &Pool<Postgres>,
    order_id: Uuid,
) -> Result<Vec<Task>, TaskError> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT id, 
            order_id,
            action as "action: ActionType",
            status as "status: TaskStatus",
            payload as "payload: TaskPayload",
            max_retries,
            task_sequence,
            scheduled_at,
            created_at, 
            updated_at
        FROM tasks
        WHERE order_id = $1
        ORDER BY task_sequence ASC
        "#,
        order_id
    )
    .fetch_all(pool)
    .await
    .map_err(TaskError::DatabaseError)
}

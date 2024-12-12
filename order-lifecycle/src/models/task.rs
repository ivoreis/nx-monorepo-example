use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use std::{fmt, str};
use uuid::Uuid;

use super::TaskPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "action_enum")]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    #[sqlx(rename = "issue")]
    Issue,
    #[sqlx(rename = "activate")]
    Activate,
    #[sqlx(rename = "suspend")]
    Suspend,
    #[sqlx(rename = "update")]
    Update,
    #[sqlx(rename = "cancel")]
    Cancel,
    #[sqlx(rename = "terminate")]
    Terminate,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task_type_str = match self {
            ActionType::Issue => "issue",
            ActionType::Activate => "ativate",
            ActionType::Suspend => "suspend",
            ActionType::Update => "update",
            ActionType::Cancel => "cancel",
            ActionType::Terminate => "terminate",
        };
        write!(f, "{}", task_type_str)
    }
}

impl str::FromStr for ActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "issue" => Ok(ActionType::Issue),
            "activate" => Ok(ActionType::Activate),
            "suspend" => Ok(ActionType::Suspend),
            "update" => Ok(ActionType::Update),
            "cancel" => Ok(ActionType::Cancel),
            "terminate" => Ok(ActionType::Terminate),
            _ => Err(format!("Invalid ActionType: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "status_enum")]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "in_progress")]
    InProgress,
    #[sqlx(rename = "completed")]
    Completed,
    #[sqlx(rename = "failed")]
    Failed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
        };
        write!(f, "{}", status_str)
    }
}

impl AsRef<str> for TaskStatus {
    fn as_ref(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
        }
    }
}

impl str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(TaskStatus::Pending),
            "in_progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            "failed" => Ok(TaskStatus::Failed),
            _ => Err(format!("Invalid TaskStatus: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Task {
    pub id: Uuid,
    pub order_id: Uuid,
    pub action: ActionType,
    pub status: TaskStatus,
    pub payload: TaskPayload,
    pub max_retries: i32,
    pub task_sequence: i32,
    pub scheduled_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

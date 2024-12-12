use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use std::fmt::{Display, Formatter, Result};
use uuid::Uuid;

use super::{Country, OrderPayload, Product};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "order_state_enum")]
pub enum OrderState {
    #[sqlx(rename = "requested")]
    Requested,
    #[sqlx(rename = "issued")]
    Issued,
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "suspended")]
    Suspended,
    #[sqlx(rename = "terminated")]
    Terminated,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

impl Display for OrderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let state_str = match self {
            OrderState::Requested => "requested",
            OrderState::Issued => "issued",
            OrderState::Active => "active",
            OrderState::Suspended => "suspended",
            OrderState::Terminated => "terminated",
            OrderState::Cancelled => "cancelled",
        };
        write!(f, "{}", state_str)
    }
}

impl AsRef<str> for OrderState {
    fn as_ref(&self) -> &str {
        match self {
            OrderState::Requested => "requested",
            OrderState::Issued => "issued",
            OrderState::Active => "active",
            OrderState::Suspended => "suspended",
            OrderState::Terminated => "terminated",
            OrderState::Cancelled => "cancelled",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Order {
    pub id: Uuid,
    pub state: OrderState,
    pub country: Country,
    pub product: Product,
    #[serde(flatten)]
    pub payload: OrderPayload,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

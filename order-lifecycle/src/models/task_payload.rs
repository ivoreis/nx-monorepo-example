use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::types::Json;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Type,
};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum ActivationReason {
    #[sqlx(rename = "initial")]
    Initial, // First time order activates
    #[sqlx(rename = "reactivation")]
    Reactivation, // Reactivation after a cancellation or Suspension
}

impl Display for ActivationReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason_str = match self {
            ActivationReason::Initial => "initial",
            ActivationReason::Reactivation => "reactivation",
        };
        write!(f, "{}", reason_str)
    }
}

impl AsRef<str> for ActivationReason {
    fn as_ref(&self) -> &str {
        match self {
            ActivationReason::Initial => "initial",
            ActivationReason::Reactivation => "reactivation",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum CancellationReason {
    #[sqlx(rename = "customer")]
    Customer,
    #[sqlx(rename = "support")]
    Support,
}

impl Display for CancellationReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason_str = match self {
            CancellationReason::Customer => "customer",
            CancellationReason::Support => "support",
        };
        write!(f, "{}", reason_str)
    }
}

impl AsRef<str> for CancellationReason {
    fn as_ref(&self) -> &str {
        match self {
            CancellationReason::Customer => "customer",
            CancellationReason::Support => "support",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum TerminationReason {
    #[sqlx(rename = "expired")]
    Expired,
    #[sqlx(rename = "fraud")]
    Fraud,
}

impl Display for TerminationReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let reason_str = match self {
            TerminationReason::Expired => "expired",
            TerminationReason::Fraud => "fraud",
        };
        write!(f, "{}", reason_str)
    }
}

impl AsRef<str> for TerminationReason {
    fn as_ref(&self) -> &str {
        match self {
            TerminationReason::Expired => "expired",
            TerminationReason::Fraud => "fraud",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TaskPayload {
    Issue {
        order_id: Uuid,
        effective_date: String,
    },
    Activate {
        order_id: Uuid,
        activation_date: String,
        activation_reason: ActivationReason,
    },
    Suspend {
        order_id: Uuid,
        reason: String,
        suspension_period: Option<String>,
    },
    Cancel {
        order_id: Uuid,
        reason: CancellationReason,
    },
    Terminate {
        order_id: Uuid,
        reason: TerminationReason,
    },
    Update {
        order_id: Uuid,
        updates: serde_json::Value,
    },
}

impl Display for TaskPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let payload_str = match self {
            TaskPayload::Issue {
                order_id,
                effective_date,
            } => format!(
                "Issue: order_id={}, effective_date={}",
                order_id, effective_date
            ),
            TaskPayload::Activate {
                order_id,
                activation_date,
                activation_reason,
            } => format!(
                "Activate: order_id={}, activation_date={}, activation_reason={}",
                order_id, activation_date, activation_reason
            ),
            TaskPayload::Suspend {
                order_id,
                reason,
                suspension_period,
            } => format!(
                "Suspend: order_id={}, reason={}, suspension_period={:?}",
                order_id, reason, suspension_period
            ),
            TaskPayload::Cancel { order_id, reason } => {
                format!("Suspend: order_id={}, reason={}", order_id, reason)
            }
            TaskPayload::Update { order_id, updates } => {
                format!("Update: order_id={}, updates={}", order_id, updates)
            }
            TaskPayload::Terminate { order_id, reason } => {
                format!("Suspend: order_id={}, reason={}", order_id, reason)
            }
        };

        write!(f, "{}", payload_str)
    }
}

impl<'r> Decode<'r, sqlx::Postgres> for TaskPayload {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let json: Json<serde_json::Value> = Decode::decode(value)?;
        serde_json::from_value(json.0)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

impl<'q> Encode<'q, sqlx::Postgres> for TaskPayload {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn Error + Send + Sync + 'static)>> {
        let json = Json(serde_json::to_value(self).unwrap());
        json.encode_by_ref(buf)
    }
}

impl Type<sqlx::Postgres> for TaskPayload {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        Json::<serde_json::Value>::type_info() // Use Json's type info
    }
}

use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::types::Json;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Type,
};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderPayload {
    #[serde(rename = "payload")]
    Car {
        #[serde(flatten)]
        details: CarPayload,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CarPayload {
    #[serde(rename_all = "snake_case")]
    #[serde(rename = "uk:car")]
    UK {
        version: i32,
        model: String,
        brand: String,
        color: String,
        price: i32,
    },
}

impl<'r> Decode<'r, sqlx::Postgres> for OrderPayload {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let json: Json<serde_json::Value> = Decode::decode(value)?;
        serde_json::from_value(json.0)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

impl<'q> Encode<'q, sqlx::Postgres> for OrderPayload {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn Error + Send + Sync + 'static)>> {
        let json = Json(serde_json::to_value(self).unwrap());
        json.encode_by_ref(buf)
    }
}

impl Type<sqlx::Postgres> for OrderPayload {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        Json::<serde_json::Value>::type_info() // Use Json's type info
    }
}

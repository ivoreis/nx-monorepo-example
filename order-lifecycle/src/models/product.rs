use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "product_enum")]
pub enum Product {
    #[sqlx(rename = "car")]
    CAR,
}

impl AsRef<str> for Product {
    fn as_ref(&self) -> &str {
        match self {
            &Product::CAR => "car",
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let product_str = match self {
            Product::CAR => "car",
        };
        write!(f, "{}", product_str)
    }
}

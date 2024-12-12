use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "country_enum")]
pub enum Country {
    #[sqlx(rename = "uk")]
    UK,
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        match self {
            Country::UK => "uk",
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let country_str = match self {
            Country::UK => "uk",
        };
        write!(f, "{}", country_str)
    }
}

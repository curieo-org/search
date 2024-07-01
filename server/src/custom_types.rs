use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(#[serde(with = "time::serde::rfc3339")] pub time::OffsetDateTime);

impl From<time::OffsetDateTime> for DateTime {
    fn from(value: time::OffsetDateTime) -> Self {
        DateTime(value)
    }
}

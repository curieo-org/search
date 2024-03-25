use std::error::Error;
use std::fmt::{Debug, Display};

use serde::{Deserialize, Deserializer};
use sqlx::{Decode, Postgres};
use sqlx::database::HasValueRef;

/// A wrapper around a value that should be kept secret
/// when displayed. This is useful for fields like passwords
/// and access tokens. The value is redacted when displayed
/// or debugged.
#[derive(Default, Clone)]
pub struct Secret<T>(pub T)
where
    T: Default + Clone;

impl<T> Display for Secret<T>
where
    T: Default + Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

impl<T> Debug for Secret<T>
where
    T: Default + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

impl<T> sqlx::Type<Postgres> for Secret<T>
where
    T: Default + Clone + sqlx::Type<Postgres>,
{
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <T as sqlx::Type<Postgres>>::type_info()
    }
}

impl<T> sqlx::Decode<'_, Postgres> for Secret<T>
where
    for<'a> T: sqlx::Type<Postgres> + sqlx::Decode<'a, Postgres> + Default + Clone,
{
    fn decode(
        value: <Postgres as HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <T as Decode<Postgres>>::decode(value)?;
        Ok(Secret(value))
    }
}

impl<'de, T> Deserialize<'de> for Secret<T>
where
    T: Deserialize<'de> + Default + Clone + Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Secret)
    }
}

impl<T> AsRef<T> for Secret<T>
where
    T: Default + Clone,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Secret<T>
where
    T: Default + Clone,
{
    fn from(s: T) -> Self {
        Self(s)
    }
}

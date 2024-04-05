use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::database::HasValueRef;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::{Debug, Display};

/// A wrapper around a value that should be kept secret
/// when displayed. This is useful for fields like passwords
/// and access tokens. The value is redacted when displayed
/// or debugged.
pub struct Secret<T>(T);

impl<T> Clone for Secret<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Secret(self.0.clone())
    }
}

impl<T> Default for Secret<T>
where
    T: Default,
{
    fn default() -> Self {
        Secret(T::default())
    }
}

impl<T> Secret<T> {
    pub fn new(t: T) -> Secret<T> {
        Secret(t)
    }
    pub fn expose(&self) -> &T {
        &self.0
    }
    pub fn expose_owned(self) -> T {
        self.0
    }
}

impl<T> Display for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

impl<T> Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[redacted]")
    }
}

impl<T> sqlx::Type<Postgres> for Secret<T>
where
    T: sqlx::Type<Postgres>,
{
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <T as sqlx::Type<Postgres>>::type_info()
    }
}

impl<T> sqlx::Decode<'_, Postgres> for Secret<T>
where
    for<'a> T: sqlx::Type<Postgres> + sqlx::Decode<'a, Postgres>,
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
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Secret)
    }
}

impl<T> Serialize for Secret<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We want to skip serializing the secret value
        // when serializing to JSON
        serializer.serialize_unit()
    }
}

impl<T> From<T> for Secret<T> {
    fn from(s: T) -> Self {
        Self(s)
    }
}

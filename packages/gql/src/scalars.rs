use async_graphql::scalar;
use piteo_core::chrono;
use piteo_core::decimal;
use serde::Deserialize;
use serde::Serialize;

// # Scalars. https://async-graphql.github.io/async-graphql/en/custom_scalars.html

scalar!(AuthId, "AuthID");

scalar!(DateTime);

scalar!(Decimal);

#[derive(Serialize, Deserialize)]
pub struct AuthId(String);

impl From<piteo_core::AuthId> for AuthId {
    fn from(item: piteo_core::AuthId) -> Self {
        Self(item.inner().to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateTime(chrono::NaiveDateTime);

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(item: chrono::NaiveDateTime) -> Self {
        Self(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decimal(decimal::Decimal);

impl From<decimal::Decimal> for Decimal {
    fn from(item: decimal::Decimal) -> Self {
        Self(item)
    }
}
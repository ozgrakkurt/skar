use super::hex::{decode_hex, encode_hex_quantity};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Quantity(Box<[u8]>);

impl From<Vec<u8>> for Quantity {
    fn from(buf: Vec<u8>) -> Self {
        Self(buf.into())
    }
}

impl From<&[u8]> for Quantity {
    fn from(buf: &[u8]) -> Self {
        Self(buf.into())
    }
}

struct QuantityVisitor;

impl<'de> Visitor<'de> for QuantityVisitor {
    type Value = Quantity;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for a quantity")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let buf: Vec<u8> = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

        Ok(Quantity::from(buf))
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(QuantityVisitor)
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex_quantity(self))
    }
}

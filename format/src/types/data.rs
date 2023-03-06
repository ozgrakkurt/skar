use super::hex::{decode_hex, encode_hex_data};
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
pub struct Data(Box<[u8]>);

impl From<Vec<u8>> for Data {
    fn from(buf: Vec<u8>) -> Self {
        Self(buf.into())
    }
}

impl From<&[u8]> for Data {
    fn from(buf: &[u8]) -> Self {
        Self(buf.into())
    }
}

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = Data;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for data")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let buf: Vec<u8> = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

        Ok(Data::from(buf))
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex_data(self))
    }
}

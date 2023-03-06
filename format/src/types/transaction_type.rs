use crate::{Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Legacy,
    AccessListType,
    DynamicFee,
}

impl FromStr for TransactionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "0x0" => Ok(Self::Legacy),
            "0x1" => Ok(Self::AccessListType),
            "0x2" => Ok(Self::DynamicFee),
            _ => Err(Error::UnknownTransactionType(s.to_owned())),
        }
    }
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Legacy => "0x0",
            Self::AccessListType => "0x1",
            Self::DynamicFee => "0x2",
        }
    }
}

struct TransactionTypeVisitor;

impl<'de> Visitor<'de> for TransactionTypeVisitor {
    type Value = TransactionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for transaction type")
    }

    fn visit_str<E>(self, value: &str) -> StdResult<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionType::from_str(value).map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TransactionTypeVisitor)
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

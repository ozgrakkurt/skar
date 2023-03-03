use crate::{Address, BloomFilter, Bytes32, Index, Nonce, Status, TransactionType};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// @@@@@@@@@@@@@@@@@@@@@@@ Index @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

struct IndexVisitor;

impl<'de> Visitor<'de> for IndexVisitor {
    type Value = Index;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for integer")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u64::from_str_radix(value, 16)
            .map_err(|e| E::custom(e.to_string()))
            .map(Into::into)
    }
}

impl<'de> Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IndexVisitor)
    }
}

impl Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// @@@@@@@@@@@@@@@@@@@@@@@ Status @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for status")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "0x1" => Ok(Status::Success),
            "0x0" => Ok(Status::Failure),
            _ => Err(E::custom("unknown status")),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(StatusVisitor)
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Status::Success => "0x1",
            Status::Failure => "0x0",
        };
        serializer.serialize_str(s)
    }
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// @@@@@@@@@@@@@@@@@@@@@@@ TransactionType @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

struct TransactionTypeVisitor;

impl<'de> Visitor<'de> for TransactionTypeVisitor {
    type Value = TransactionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for transaction type")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "0x0" => Ok(TransactionType::Legacy),
            "0x1" => Ok(TransactionType::AccessListType),
            "0x2" => Ok(TransactionType::DynamicFee),
            _ => Err(E::custom("unknown transaction type")),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TransactionTypeVisitor)
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            TransactionType::Legacy => "0x0",
            TransactionType::AccessListType => "0x1",
            TransactionType::DynamicFee => "0x2",
        };
        serializer.serialize_str(s)
    }
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

macro_rules! impl_serde_for_bytes {
    ($bytes:ident, $visitor:ident, $length:expr) => {
        struct $visitor;

        impl<'de> Visitor<'de> for $visitor {
            type Value = $bytes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(concat!("hex string for ", $length, " byte value"))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let buf: [u8; $length] =
                    prefix_hex::decode(value).map_err(|e| E::custom(e.to_string()))?;

                Ok(Box::new(buf).into())
            }
        }

        impl<'de> Deserialize<'de> for $bytes {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor)
            }
        }

        impl Serialize for $bytes {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let hex = prefix_hex::encode(self.as_ref());

                serializer.serialize_str(&hex)
            }
        }
    };
}

impl_serde_for_bytes!(Bytes32, Bytes32Visitor, 32);
impl_serde_for_bytes!(Address, AddressVisitor, 20);
impl_serde_for_bytes!(BloomFilter, BloomFilterVisitor, 256);
impl_serde_for_bytes!(Nonce, NonceVisitor, 8);

#[cfg(test)]
mod tests {
    use super::*;
}

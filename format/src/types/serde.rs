use crate::Error;
use crate::{Address, BloomFilter, Bytes, Bytes32, Index, Quantity, Status, TransactionType};
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
        let value = value
            .strip_prefix("0x")
            .ok_or_else(|| E::custom("invalid hex prefix"))?;

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
        serializer.serialize_str(&encode_hex_int(&self.to_be_bytes()))
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

// @@@@@@@@@@@@@@@@@@@@@@@ Bytes @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let buf: Vec<u8> = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

        Ok(Bytes::from(buf))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BytesVisitor)
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex(self))
    }
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// @@@@@@@@@@@@@@@@@@@@@@@ Quantity @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

struct QuantityVisitor;

impl<'de> Visitor<'de> for QuantityVisitor {
    type Value = Quantity;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for QUANTITY")
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
        serializer.serialize_str(&encode_hex_int(self.as_ref()))
    }
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

macro_rules! impl_serde_for_fixed_size_bytes {
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
                let buf: [u8; $length] = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

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
                let hex = format!("0x{}", hex::encode(self.as_ref()));
                serializer.serialize_str(&hex)
            }
        }
    };
}

impl_serde_for_fixed_size_bytes!(Bytes32, Bytes32Visitor, 32);
impl_serde_for_fixed_size_bytes!(Address, AddressVisitor, 20);
impl_serde_for_fixed_size_bytes!(BloomFilter, BloomFilterVisitor, 256);

fn decode_hex<T>(value: &str) -> Result<T, Error>
where
    T: hex::FromHex<Error = hex::FromHexError>,
{
    let value = value.strip_prefix("0x").ok_or(Error::InvalidHexPrefix)?;

    let buf: T = if value.len() % 2 != 0 {
        let value = format!("0{}", &value);
        T::from_hex(value).map_err(Error::DecodeHex)?
    } else {
        T::from_hex(value).map_err(Error::DecodeHex)?
    };

    Ok(buf)
}

fn encode_hex_impl(buf: &[u8], default_hex: &'static str) -> String {
    let hex = hex::encode(buf);

    match hex.find(|c| c != '0') {
        Some(idx) => format!("0x{}", &hex[idx..]),
        None => default_hex.to_owned(),
    }
}

fn encode_hex(buf: &[u8]) -> String {
    encode_hex_impl(buf, "0x")
}

fn encode_hex_int(buf: &[u8]) -> String {
    encode_hex_impl(buf, "0x0")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{de::DeserializeOwned, Serialize};
    use std::fmt::Debug;

    fn roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(val: &T) {
        let other = serde_json::from_str(&serde_json::to_string(&val).unwrap()).unwrap();

        assert_eq!(val, &other);
    }

    fn ignore_leading_zeroes_roundtrip<
        T: Serialize + DeserializeOwned + PartialEq + Debug + Into<Box<[u8]>>,
    >(
        val: T,
    ) {
        let other: T = serde_json::from_str(&serde_json::to_string(&val).unwrap()).unwrap();

        let val: Box<[u8]> = val.into();

        let val = match val.as_ref().iter().enumerate().find(|(_, b)| **b != 0) {
            Some((idx, _)) => &val.as_ref()[idx..],
            None => &[0],
        };

        assert_eq!(val, other.into().as_ref());
    }

    #[test]
    fn test_index_roundtrip() {
        roundtrip(&Index::default());
        roundtrip(&Index::from(0));
        roundtrip(&Index::from(std::u64::MAX));
        roundtrip(&Index::from(25));
    }

    #[test]
    fn test_index_serialize() {
        assert_eq!(&serde_json::to_string(&Index::from(0)).unwrap(), "\"0x0\"");
        assert_eq!(&serde_json::to_string(&Index::from(1)).unwrap(), "\"0x1\"");
        assert_eq!(
            &serde_json::to_string(&Index::from(25)).unwrap(),
            "\"0x19\""
        );
    }

    #[test]
    fn test_status_roundtrip() {
        roundtrip(&Status::Success);
        roundtrip(&Status::Failure);
    }

    #[test]
    fn test_transaction_type_roundtrip() {
        roundtrip(&TransactionType::Legacy);
        roundtrip(&TransactionType::AccessListType);
        roundtrip(&TransactionType::DynamicFee);
    }

    #[test]
    fn test_bytes32_roundtrip() {
        roundtrip(&Bytes32::default());
        roundtrip(&Bytes32::try_from((0..32).collect::<Vec<u8>>().as_slice()).unwrap());
    }

    #[test]
    fn test_bytes32_zeroes_in_middle_roundtrip() {
        roundtrip(
            &Bytes32::try_from(
                (0..15)
                    .chain(std::iter::repeat(0).take(2))
                    .chain(0..15)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_bytes32_trailing_zeroes_roundtrip() {
        roundtrip(
            &Bytes32::try_from(
                (0..15)
                    .chain(std::iter::repeat(0).take(17))
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_bytes32_leading_zeroes_roundtrip() {
        roundtrip(
            &Bytes32::try_from(
                std::iter::repeat(0)
                    .take(16)
                    .chain(0..16)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_bytes_roundtrip() {
        ignore_leading_zeroes_roundtrip(Bytes::default());
        ignore_leading_zeroes_roundtrip(Bytes::from((1..32).collect::<Vec<u8>>().as_slice()));
    }

    #[test]
    fn test_bytes_zeroes_in_middle_roundtrip() {
        ignore_leading_zeroes_roundtrip(Bytes::from(
            (3..15)
                .chain(std::iter::repeat(3).take(2))
                .chain(0..15)
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_bytes_trailing_zeroes_roundtrip() {
        ignore_leading_zeroes_roundtrip(Bytes::from(
            (1..15)
                .chain(std::iter::repeat(0).take(17))
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_bytes_leading_zeroes_roundtrip() {
        ignore_leading_zeroes_roundtrip(Bytes::from(
            std::iter::repeat(0)
                .take(16)
                .chain(0..16)
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_bytes_empty_serialize() {
        assert_eq!(&serde_json::to_string(&Bytes::default()).unwrap(), "\"0x\"");
    }

    #[test]
    fn test_quantity_roundtrip() {
        ignore_leading_zeroes_roundtrip(Quantity::default());
        ignore_leading_zeroes_roundtrip(Quantity::from((1..32).collect::<Vec<u8>>().as_slice()));
    }

    #[test]
    fn test_quantity_zeroes_in_middle_roundtrip() {
        ignore_leading_zeroes_roundtrip(Quantity::from(
            (3..15)
                .chain(std::iter::repeat(3).take(2))
                .chain(0..15)
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_quantity_trailing_zeroes_roundtrip() {
        ignore_leading_zeroes_roundtrip(Quantity::from(
            (1..15)
                .chain(std::iter::repeat(0).take(17))
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_quantity_leading_zeroes_roundtrip() {
        ignore_leading_zeroes_roundtrip(Quantity::from(
            std::iter::repeat(0)
                .take(16)
                .chain(0..16)
                .collect::<Vec<u8>>()
                .as_slice(),
        ));
    }

    #[test]
    fn test_quantity_empty_serialize() {
        assert_eq!(
            &serde_json::to_string(&Quantity::default()).unwrap(),
            "\"0x0\""
        );
    }
}

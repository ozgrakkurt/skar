use super::hex::{decode_hex, encode_hex_fixed_size_data};
use crate::{Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::result::Result as StdResult;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct FixedSizeData<const N: usize>(Box<[u8; N]>);

impl<const N: usize> Default for FixedSizeData<N> {
    fn default() -> Self {
        Self(Box::new([0; N]))
    }
}

impl<const N: usize> TryFrom<&[u8]> for FixedSizeData<N> {
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<FixedSizeData<N>> {
        let buf: [u8; N] = buf.try_into().map_err(|_| Error::UnexpectedLength {
            expected: N,
            got: buf.len(),
        })?;

        Ok(FixedSizeData(Box::new(buf)))
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for FixedSizeData<N> {
    type Error = Error;

    fn try_from(buf: Vec<u8>) -> Result<FixedSizeData<N>> {
        let len = buf.len();
        let buf: Box<[u8; N]> = buf.try_into().map_err(|_| Error::UnexpectedLength {
            expected: N,
            got: len,
        })?;

        Ok(FixedSizeData(buf))
    }
}

struct FixedSizeDataVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for FixedSizeDataVisitor<N> {
    type Value = FixedSizeData<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("hex string for {N} byte data"))
    }

    fn visit_str<E>(self, value: &str) -> StdResult<Self::Value, E>
    where
        E: de::Error,
    {
        let buf = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

        Self::Value::try_from(buf).map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedSizeData<N> {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FixedSizeDataVisitor)
    }
}

impl<const N: usize> Serialize for FixedSizeData<N> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex_fixed_size_data(self.as_ref()))
    }
}

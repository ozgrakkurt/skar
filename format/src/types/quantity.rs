use crate::{Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;
use std::result::Result as StdResult;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Quantity(Box<[u8]>);

impl From<Vec<u8>> for Quantity {
    fn from(buf: Vec<u8>) -> Self {
        assert!(!buf.is_empty());
        assert!(buf.len() == 1 || buf[0] != 0);

        Self(buf.into())
    }
}

impl From<&[u8]> for Quantity {
    fn from(buf: &[u8]) -> Self {
        assert!(!buf.is_empty());
        assert!(buf.len() == 1 || buf[0] != 0);

        Self(buf.into())
    }
}

impl Default for Quantity {
    fn default() -> Quantity {
        Quantity(Box::new([0]))
    }
}

impl<const N: usize> From<[u8; N]> for Quantity {
    fn from(buf: [u8; N]) -> Self {
        Self(buf.into())
    }
}

struct QuantityVisitor;

impl<'de> Visitor<'de> for QuantityVisitor {
    type Value = Quantity;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for a quantity")
    }

    fn visit_str<E>(self, value: &str) -> StdResult<Self::Value, E>
    where
        E: de::Error,
    {
        let buf: Vec<u8> = decode_hex(value).map_err(|e| E::custom(e.to_string()))?;

        Ok(Quantity::from(buf))
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(QuantityVisitor)
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex(self))
    }
}

fn decode_hex(value: &str) -> Result<Vec<u8>> {
    if value == "0x0" {
        return Ok(vec![0]);
    }

    let val = value
        .strip_prefix("0x")
        .ok_or_else(|| Error::InvalidHexPrefix(value.to_owned()))?;

    if val.starts_with('0') {
        return Err(Error::UnexpectedLeadingZeroes(value.to_owned()));
    }

    let mut val: Cow<_> = val.into();

    if val.len() % 2 != 0 {
        val = format!("0{val}").into();
    }

    hex::decode(val.as_ref()).map_err(Error::DecodeHex)
}

pub fn encode_hex(buf: &[u8]) -> Cow<'static, str> {
    let hex_val = hex::encode(buf);

    match hex_val.find(|c| c != '0') {
        Some(idx) => format!("0x{}", &hex_val[idx..]).into(),
        None => "0x0".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::Quantity;
    use hex_literal::hex;
    use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

    #[test]
    fn test_serde_empty() {
        assert_tokens(&Quantity::default(), &[Token::Str("0x0")]);
    }

    #[test]
    fn test_serialize() {
        assert_ser_tokens(&Quantity::from(hex!("004200")), &[Token::Str("0x4200")]);
        assert_ser_tokens(&Quantity::from(hex!("420000")), &[Token::Str("0x420000")]);
        assert_ser_tokens(&Quantity::from(hex!("000042")), &[Token::Str("0x42")]);
    }

    #[test]
    fn test_deserialize() {
        assert_de_tokens(&Quantity::from(hex!("420000")), &[Token::Str("0x420000")]);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_leading_zeroes() {
        assert_de_tokens(
            &Quantity::from(hex!("00420000")),
            &[Token::Str("0x00420000")],
        );
    }
}

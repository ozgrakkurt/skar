use super::quantity::encode_hex;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::result::Result as StdResult;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct UInt(u64);

struct UIntVisitor;

impl<'de> Visitor<'de> for UIntVisitor {
    type Value = UInt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("hex string for integer")
    }

    fn visit_str<E>(self, value: &str) -> StdResult<Self::Value, E>
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

impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UIntVisitor)
    }
}

impl Serialize for UInt {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex(&self.to_be_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::UInt;
    use serde_test::{assert_de_tokens, assert_tokens, Token};

    #[test]
    fn test_serde_zero() {
        assert_eq!(UInt::default(), UInt::from(0));

        assert_tokens(&UInt::from(0), &[Token::Str("0x0")]);
    }

    #[test]
    fn test_serde_max() {
        assert_tokens(
            &UInt::from(std::u64::MAX),
            &[Token::Str("0xffffffffffffffff")],
        );
    }

    #[test]
    fn test_serde() {
        assert_tokens(&UInt::from(19), &[Token::Str("0x13")]);
    }

    #[test]
    #[should_panic(expected = "number too large")]
    fn test_serde_overflow() {
        assert_de_tokens(&UInt::from(19), &[Token::Str("0xffffffffffffffffa")]);
    }
}

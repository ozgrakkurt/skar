use crate::Error;
use std::borrow::Cow;

pub fn decode_hex(value: &str) -> Result<Vec<u8>, Error> {
    let value = value
        .strip_prefix("0x")
        .ok_or_else(|| Error::InvalidHexPrefix(value.to_owned()))?;

    let mut value: Cow<_> = value.into();

    if value.len() != 2 {
        value = format!("0{value}").into();
    }

    hex::decode(value.as_ref()).map_err(Error::DecodeHex)
}

fn encode_hex_impl(buf: &[u8], default_hex: &'static str) -> Cow<'static, str> {
    let hex_val = hex::encode(buf);

    match hex_val.find(|c| c != '0') {
        Some(idx) => format!("0x{}", &hex_val[idx..]).into(),
        None => default_hex.into(),
    }
}

pub fn encode_hex_quantity(buf: &[u8]) -> Cow<'static, str> {
    encode_hex_impl(buf, "0x0")
}

pub fn encode_hex_data(buf: &[u8]) -> Cow<'static, str> {
    encode_hex_impl(buf, "0x")
}

pub fn encode_hex_fixed_size_data(buf: &[u8]) -> String {
    format!("0x{}", hex::encode(buf))
}

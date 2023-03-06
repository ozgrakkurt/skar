use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Invalid array length. Expected {0} got {1}.")]
    InvalidArrayLength(usize, usize),
    #[error("Failed to decode hex string:\n{0}")]
    DecodeHex(hex::FromHexError),
    #[error("Invalid hex prefix. Hex string doesn't start with \"0x\"")]
    InvalidHexPrefix,
}

pub type Result<T> = StdResult<T, Error>;

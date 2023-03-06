use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use std::array::TryFromSliceError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("array from slice error:\n{0}")]
    ArrayFromSlice(TryFromSliceError),
    #[error("Unexpected length. Expected {expected} got {got}.")]
    UnexpectedLength {
        expected: usize,
        got: usize,
    },
    #[error("Failed to decode hex string:\n{0}")]
    DecodeHex(hex::FromHexError),
    #[error("Invalid hex prefix. Hex string doesn't start with \"0x\"")]
    InvalidHexPrefix,
}

pub type Result<T> = StdResult<T, Error>;

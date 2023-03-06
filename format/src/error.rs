use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Unexpected length. Expected {expected} got {got}.")]
    UnexpectedLength { expected: usize, got: usize },
    #[error("Failed to decode hex string:\n{0}")]
    DecodeHex(hex::FromHexError),
    #[error("Invalid hex prefix. Hex string doesn't start with \"0x\". Value was: \"{0}\"")]
    InvalidHexPrefix(String),
    #[error("Unknown transaction status: {0}")]
    UnknownTransactionStatus(String),
    #[error("Unknown transaction type: {0}")]
    UnknownTransactionType(String),
    #[error("Unexpected leading zeroes. Value was: {0}")]
    UnexpectedLeadingZeroes(String),
}

pub type Result<T> = StdResult<T, Error>;

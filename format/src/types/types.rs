use crate::{Error, Result};
use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub number: BlockNumber,
    pub hash: Hash,
    pub parent_hash: Hash,
    pub nonce: Nonce,
    pub sha3_uncles: Hash,
    pub logs_bloom: BloomFilter,
    pub transactions_root: Hash,
    pub state_root: Hash,
    pub receipts_root: Hash,
    pub miner: Address,
    pub difficulty: Quantity,
    pub total_difficulty: Quantity,
    pub extra_data: Bytes,
    pub size: Quantity,
    pub gas_limit: Quantity,
    pub gas_used: Quantity,
    pub timestamp: Quantity,
    pub uncles: Box<[Hash]>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(flatten)]
    pub header: BlockHeader,
    pub transactions: Box<[Transaction]>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub from: Address,
    pub gas: Quantity,
    pub gas_price: Quantity,
    pub hash: Hash,
    pub input: Bytes,
    pub nonce: Quantity,
    pub to: Option<Address>,
    pub transaction_index: TransactionIndex,
    pub value: Quantity,
    pub v: Quantity,
    pub r: Quantity,
    pub s: Quantity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: Hash,
    pub transaction_index: TransactionIndex,
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub from: Address,
    pub to: Option<Address>,
    pub cumulative_gas_used: Quantity,
    pub effective_gas_price: Quantity,
    pub gas_used: Quantity,
    pub contract_address: Option<Address>,
    pub logs: Box<[Log]>,
    pub logs_bloom: BloomFilter,
    #[serde(rename = "type")]
    pub kind: TransactionType,
    pub root: Option<Hash>,
    pub status: Option<Status>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub removed: bool,
    pub log_index: LogIndex,
    pub transaction_index: TransactionIndex,
    pub transaction_hash: Hash,
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub address: Address,
    pub data: Box<LogArgument>,
    pub topics: ArrayVec<LogArgument, 4>,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct BlockNumber(Index);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Hash(Bytes32);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Nonce(Bytes);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct BloomFilter(Box<[u8; 256]>);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Address(Box<[u8; 20]>);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Quantity(Box<[u8]>);

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct TransactionIndex(Index);

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct LogIndex(Index);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct LogArgument(Bytes32);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Bytes32(Box<[u8; 32]>);

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Bytes(Box<[u8]>);

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct Index(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Legacy,
    AccessListType,
    DynamicFee,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,
    Failure,
}

impl Default for BloomFilter {
    fn default() -> Self {
        Self(Box::new([0; 256]))
    }
}

macro_rules! impl_try_from_for_fixed_size_bytes {
    ($typename:ident, $expected_len:expr) => {
        impl TryFrom<&[u8]> for $typename {
            type Error = Error;

            fn try_from(buf: &[u8]) -> Result<Self> {
                if buf.len() != $expected_len {
                    return Err(Error::InvalidArrayLength($expected_len, buf.len()));
                }
                Ok(buf.try_into().map(Box::new).map(Self).unwrap())
            }
        }
    };
}

impl_try_from_for_fixed_size_bytes!(Address, 20);
impl_try_from_for_fixed_size_bytes!(BloomFilter, 256);
impl_try_from_for_fixed_size_bytes!(Bytes32, 32);

impl From<Vec<u8>> for Bytes {
    fn from(b: Vec<u8>) -> Bytes {
        Bytes(b.into())
    }
}

impl From<&[u8]> for Bytes {
    fn from(b: &[u8]) -> Bytes {
        Bytes(b.into())
    }
}

impl From<Vec<u8>> for Quantity {
    fn from(b: Vec<u8>) -> Quantity {
        Quantity(b.into())
    }
}

impl From<&[u8]> for Quantity {
    fn from(b: &[u8]) -> Quantity {
        Quantity(b.into())
    }
}

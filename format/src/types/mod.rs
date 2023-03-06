use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

mod data;
mod fixed_size_data;
mod quantity;
mod transaction_status;
mod transaction_type;
mod uint;

pub use data::Data;
pub use fixed_size_data::FixedSizeData;
pub use quantity::Quantity;
pub use transaction_status::TransactionStatus;
pub use transaction_type::TransactionType;

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
    pub extra_data: Data,
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
    pub input: Data,
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
    pub status: Option<TransactionStatus>,
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
    pub data: Box<[LogArgument]>,
    pub topics: ArrayVec<LogArgument, 4>,
}

pub type Hash = FixedSizeData<32>;
pub type LogArgument = FixedSizeData<32>;
pub type Address = FixedSizeData<20>;
pub type Nonce = FixedSizeData<8>;
pub type BloomFilter = FixedSizeData<256>;
pub type BlockNumber = uint::UInt;
pub type TransactionIndex = uint::UInt;
pub type LogIndex = uint::UInt;

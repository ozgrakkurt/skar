use arrayvec::ArrayVec;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub difficulty: Unsigned256,
    pub total_difficulty: Unsigned256,
    pub extra_data: Box<[u8]>,
    pub size: Unsigned256,
    pub gas_limit: Unsigned256,
    pub gas_used: Unsigned256,
    pub timestamp: Unsigned256,
    pub uncles: Box<[Hash]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Box<[Transaction]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub from: Address,
    pub gas: Unsigned256,
    pub gas_price: Unsigned256,
    pub hash: Hash,
    pub input: Box<[u8]>,
    pub nonce: Unsigned256,
    pub to: Option<Address>,
    pub transaction_index: TransactionIndex,
    pub value: Unsigned256,
    pub v: Unsigned256,
    pub r: Unsigned256,
    pub s: Unsigned256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionReceipt {
    pub transaction_hash: Hash,
    pub transaction_index: TransactionIndex,
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub from: Address,
    pub to: Option<Address>,
    pub cumulative_gas_used: Unsigned256,
    pub effective_gas_price: Unsigned256,
    pub gas_used: Unsigned256,
    pub contract_address: Option<Address>,
    pub logs: Box<[Log]>,
    pub logs_bloom: BloomFilter,
    //#[serde(rename = "type")]
    pub kind: TransactionType,
    pub root: Option<Hash>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct BlockNumber(Index);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Hash(Bytes32);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Nonce(Box<[u8; 8]>);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct BloomFilter(Box<[u8; 256]>);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Address(Box<[u8; 20]>);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Unsigned256(Box<[u8; 32]>);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct TransactionIndex(Index);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Deref,
)]
pub struct LogIndex(Index);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct LogArgument(Bytes32);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref,
)]
pub struct Bytes32(Box<[u8; 32]>);

#[derive(
    Debug,
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

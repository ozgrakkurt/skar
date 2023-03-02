use arrayvec::ArrayVec;

pub struct BlockHeader {
	pub number: Number,
	pub hash: Hash,
	pub parentHash: Hash,
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

pub struct Block {
	pub header: BlockHeader,
	pub transactions: Box<[Transaction]>,
}

pub struct Transaction {
	pub block_hash: Hash,
	pub block_number: Number,
	pub from: Address,
	pub gas: Unsigned256,
	pub gas_price: Unsigned256,
	pub hash: Hash,
	pub input: Box<[u8]>,
	pub nonce: Unsigned256,
	pub to: Option<Address>,
	pub transaction_index: Index,
	pub value: Unsigned256,
	pub v: Unsigned256,
	pub r: Unsigned256,
	pub s: Unsigned256,
}

pub struct TransactionReceipt {
	pub transaction: Transaction,
	pub cumulative_gas_used: Unsigned256,
	pub effective_gas_price: Unsigned256,
	pub gas_used: Unsigned256,
	pub contract_address: Option<Address>,
	pub logs: Box<[Log]>,
	pub logs_bloom: BloomFilter,
	pub type: TransactionType,
	pub root: Option<Hash>,
	pub status: Option<Status>,
}

pub struct Log {
	pub removed: bool,
	pub log_index: Index,
	pub transaction_index: Index,
	pub transaction_hash: Hash,
	pub block_hash: Hash,
	pub block_number: Number,
	pub address: Address,
	pub data: Box<LogArgument>,
	pub topics: ArrayVec<LogArgument; 4>,
}

pub struct Number(u64);

pub struct Hash(Bytes32);

pub struct Nonce(Box<[u8; 8]>);

pub struct BloomFilter(Box<[u8; 256]>);

pub struct Address(Box<[u8; 20]>);

pub struct Unsigned256(Box<[u8; 32]>);

pub struct TransactionIndex(Number);

pub struct LogIndex(Number);

pub struct LogArgument(Bytes32);

pub struct Bytes32(Box<[u8; 32]>);

pub enum TransactionType {
	Legacy,
	AccessListType,
	DynamicFee,
}

pub enum Status {
	Success,
	Failure,
}

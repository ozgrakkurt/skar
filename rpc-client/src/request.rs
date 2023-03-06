use skar_format::types::BlockNumber;

pub trait EthRpcRequest {}

pub struct GetBlockByNumber {
    pub number: BlockNumber,
}

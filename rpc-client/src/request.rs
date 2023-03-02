use skar_format::BlockNumber;

pub trait EthRpcRequest {}

pub struct GetBlockByNumber {
    pub number: BlockNumber,
}

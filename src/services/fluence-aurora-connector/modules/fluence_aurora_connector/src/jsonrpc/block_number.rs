use marine_rs_sdk::marine;

use crate::jsonrpc::JsonRpcReq;
use crate::jsonrpc::JSON_RPC_VERSION;

pub struct BlockNumberReq;

impl BlockNumberReq {
    pub fn new() -> JsonRpcReq<()> {
        JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: 0,
            method: "eth_blockNumber".to_string(),
            params: vec![],
        }
    }
}

#[marine]
pub struct BlockNumberResult {
    success: bool,
    result: String,
    error: Vec<String>,
}

impl BlockNumberResult {
    pub fn ok(result: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
        }
    }

    pub fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: String::new(),
        }
    }
}

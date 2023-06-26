use serde::{Deserialize, Serialize};

use crate::chain::log::Log;
use crate::curl::send_jsonrpc;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::JsonRpcReq;
use crate::jsonrpc::JSON_RPC_VERSION;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsReq {
    pub from_block: String,
    pub to_block: String,
    pub address: String,
    pub topics: Vec<String>,
}

impl GetLogsReq {
    pub fn to_jsonrpc(self, id: u32) -> JsonRpcReq<Self> {
        JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            method: "eth_getLogs".to_string(),
            params: vec![self],
        }
    }
}

pub fn get_logs(
    api_endpoint: String,
    address: String,
    from_block: String,
    to_block: String,
    topic: String,
) -> Result<Vec<Log>, RequestError> {
    let req = GetLogsReq {
        address,
        topics: vec![topic],
        from_block,
        to_block,
    };
    let response = send_jsonrpc(api_endpoint, req.to_jsonrpc(0))?;
    let logs = response.get_result()?;
    Ok(logs)
}

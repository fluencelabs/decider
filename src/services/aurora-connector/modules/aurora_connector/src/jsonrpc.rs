use serde::{Deserialize, Serialize};
use thiserror::Error;

//
const JSON_RPC_VERSION: &str = "2.0";

// We don't id `id` field, but we need to verify the response.
const JSON_RPC_ID: u32 = 0;

#[derive(Debug, Error)]
pub enum JsonRpcError {
    #[error("wrong JSON RPC version in the response: expected {JSON_RPC_VERSION}, got {0}")]
    WrongVersion(String),
    #[error("wrong JSON RPC id in the response: expected {JSON_RPC_ID}, got {0}")]
    WrongId(u32),
}

#[derive(Debug, Serialize)]
pub struct JsonRpcReq<T> {
    jsonrpc: String,
    id: u32,
    method: String,
    params: T,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResp<T> {
    jsonrpc: String,
    id: u32,
    result: T,
}

impl<T> JsonRpcResp<T> {
    pub fn get_result(self) -> Result<T, JsonRpcError> {
        if self.jsonrpc != JSON_RPC_VERSION {
            return Err(JsonRpcError::WrongVersion(self.jsonrpc));
        }
        if self.id != JSON_RPC_ID {
            return Err(JsonRpcError::WrongId(self.id));
        }

        Ok(self.result)
    }
}

#[derive(Debug, Serialize)]
pub struct GetLogsReq {
    pub from_block: String,
    pub address: String,
    pub topics: Vec<String>,
}

impl GetLogsReq {
    pub fn to_jsonrpc(self) -> JsonRpcReq<Vec<Self>> {
        JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: 0,
            method: "eth_getLogs".to_string(),
            params: vec![self],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResp {
    // Actual data that holds all the info about the deal.
    pub data: String,
    // The block number with the deal.
    pub block_number: String,
    //
    pub removed: bool,
}

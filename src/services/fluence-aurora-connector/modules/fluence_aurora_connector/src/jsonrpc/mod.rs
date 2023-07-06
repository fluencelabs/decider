use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod block_number;
pub mod deal_changed;
pub mod deal_changed_batch;
pub mod deal_created;
pub mod deal_matched;
pub mod get_logs;
pub mod request;
pub mod right_boundary;

#[cfg(test)]
mod tests;

const JSON_RPC_VERSION: &str = "2.0";

#[derive(Debug, Error)]
pub enum JsonRpcError {
    #[error("wrong JSON RPC version in the response: expected {JSON_RPC_VERSION}, got {0}")]
    WrongVersion(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcReq<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResp<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub result: T,
}

impl<T> JsonRpcResp<T> {
    pub fn get_result(self) -> Result<T, JsonRpcError> {
        if self.jsonrpc != JSON_RPC_VERSION {
            return Err(JsonRpcError::WrongVersion(self.jsonrpc));
        }
        Ok(self.result)
    }
}

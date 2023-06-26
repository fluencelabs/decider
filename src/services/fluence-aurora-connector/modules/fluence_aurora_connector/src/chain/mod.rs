use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::chain::chain_data::ChainData;
use crate::chain::chain_event::ChainEvent;

pub mod chain_data;
pub mod chain_event;
pub mod deal;
pub mod deal_changed;
pub mod deal_created;
pub mod u256;

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
    pub params: T,
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

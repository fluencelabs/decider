use std::str::FromStr;
use libp2p_identity::{ParseError, PeerId};
use marine_rs_sdk::get_call_parameters;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
mod tests;

pub mod block_number;
pub mod deal_changed;
pub mod deal_changed_batch;
pub mod deal_created;
pub mod deal_matched;
pub mod get_logs;
pub mod register_worker;
pub mod request;
pub mod resolve_subnet;
pub mod right_boundary;
pub mod transaction;

pub mod deal_status;
pub mod deal_peer_removed_batched;

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

pub fn get_encoded_peer_id() -> Result<String, ParseError> {
    let host = get_call_parameters().host_id;
    let host = PeerId::from_str(&host)?;
    Ok(peer_id_to_topic(host))
}


pub fn peer_id_to_topic(host: PeerId) -> String {
    let host: Vec<_> = host.to_bytes().into_iter().skip(6).collect();
    format!("0x{:0>64}", hex::encode(host))
}

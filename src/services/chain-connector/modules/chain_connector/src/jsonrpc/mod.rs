use libp2p_identity::{ParseError, PeerId};
use marine_rs_sdk::get_call_parameters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use std::str::FromStr;
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

pub mod deal_peer_removed_batched;
pub mod deal_status;

const JSON_RPC_VERSION: &str = "2.0";

#[derive(Debug, Error)]
pub enum JsonRpcError {
    #[error("wrong JSON RPC version in the response: expected {JSON_RPC_VERSION}, got {0}")]
    WrongVersion(String),
    #[error("Chain JSON RPC error: {0}")]
    RpcError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcReq<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRespError {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl Display for JsonRpcRespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{} (code {})", message, self.code)?;
        } else if let Some(data) = &self.data {
            write!(f, "code {} (data {})", self.code, data)?;
        } else {
            write!(f, "code {}", self.code)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcRespResult<T> {
    Success { result: T },
    Error { error: JsonRpcRespError },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResp<T> {
    jsonrpc: String,
    id: u32,
    #[serde(flatten)]
    result: JsonRpcRespResult<T>,
}

impl<T> JsonRpcResp<T> {
    pub fn get_result(self) -> Result<T, JsonRpcError> {
        if self.jsonrpc != JSON_RPC_VERSION {
            return Err(JsonRpcError::WrongVersion(self.jsonrpc));
        }
        match self.result {
            JsonRpcRespResult::Success { result } => Ok(result),
            JsonRpcRespResult::Error { error } => Err(JsonRpcError::RpcError(error.to_string())),
        }
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

#[cfg(test)]
mod test {
    #[test]
    fn test_jsonrpc_response_parse() {
        let response_err = r#"{
            "id": 1337,
            "jsonrpc": "2.0",
            "error": {
                "code": -32003,
                "message": "Transaction rejected"
            }
        }"#;
        let response_ok = r#"{
            "id":64,
            "jsonrpc": "2.0",
            "result": "0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad"
        }"#;
        let response_batch = format!("[{response_ok},{response_err}]");

        let x = serde_json::from_str::<super::JsonRpcResp<String>>(response_err);
        assert!(x.is_ok(), "can't parse error json rpc response");
        let x = x.unwrap().get_result();
        assert!(x.is_err());

        let x = serde_json::from_str::<super::JsonRpcResp<String>>(response_ok);
        assert!(x.is_ok(), "can't parse normal json rpc response");
        let x = x.unwrap().get_result();
        assert!(x.is_ok());

        let x = serde_json::from_str::<Vec<super::JsonRpcResp<String>>>(&response_batch);
        assert!(x.is_ok(), "can't parse batch json rpc response");
    }
}

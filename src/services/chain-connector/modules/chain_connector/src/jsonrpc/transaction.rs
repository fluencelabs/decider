use crate::curl::send_jsonrpc;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::JsonRpcReq;
use crate::jsonrpc::JsonRpcResp;
use crate::jsonrpc::JSON_RPC_VERSION;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::JsonRpcError;

#[marine]
pub struct TxStatusResult {
    success: bool,
    error: Vec<String>,
    status: String,
}

impl TxStatusResult {
    fn ok() -> Self {
        Self {
            success: true,
            error: vec![],
            status: "ok".to_string(),
        }
    }

    fn pending() -> Self {
        Self {
            success: true,
            error: vec![],
            status: "pending".to_string(),
        }
    }

    fn failed() -> Self {
        Self {
            success: true,
            error: vec![],
            status: "failed".to_string(),
        }
    }

    fn error(msg: String) -> Self {
        Self {
            success: false,
            error: vec![msg],
            status: "failed".to_string(),
        }
    }
}

enum TxStatus {
    Failed,
    Ok,
    Pending,
}

#[derive(Debug, Error)]
enum TxError {
    #[error(transparent)]
    JsonRpcError(#[from] JsonRpcError),
    #[error(transparent)]
    RequestError(#[from] RequestError),
    #[error("unknown transaction status `{0}`")]
    UnexpectedStatus(String),
}

#[marine]
pub fn get_tx_status(api_endpoint: String, tx_hash: String) -> TxStatusResult {
    let req = TxReq::new(tx_hash).to_jsonrpc(0);

    let result: Result<TxStatus, TxError> = try {
        let result: JsonRpcResp<Option<TxResp>> = send_jsonrpc(&api_endpoint, req)?;
        let result = result.get_result()?;
        log::debug!("result {:?}", result);
        if let Some(result) = result {
            match result.status.as_str() {
                "0x1" => Ok(TxStatus::Ok),
                "0x0" => Ok(TxStatus::Failed),
                x => Err(TxError::UnexpectedStatus(x.to_string())),
            }?
        } else {
            TxStatus::Pending
        }
    };
    match result {
        Err(err) => TxStatusResult::error(err.to_string()),
        Ok(TxStatus::Ok) => TxStatusResult::ok(),
        Ok(TxStatus::Pending) => TxStatusResult::pending(),
        Ok(TxStatus::Failed) => TxStatusResult::failed(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TxResp {
    status: String,
}

#[derive(Serialize, Deserialize)]
struct TxReq(String);
impl TxReq {
    pub fn new(tx_hash: String) -> Self {
        Self(tx_hash)
    }

    pub fn to_jsonrpc(self, id: u32) -> JsonRpcReq<Self> {
        JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            method: "eth_getTransactionReceipt".to_string(),
            params: vec![self],
        }
    }
}

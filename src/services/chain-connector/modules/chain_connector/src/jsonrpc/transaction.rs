use crate::curl::send_jsonrpc_batch;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::JsonRpcError;
use crate::jsonrpc::JsonRpcReq;
use crate::jsonrpc::JsonRpcResp;
use crate::jsonrpc::JSON_RPC_VERSION;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[marine]
pub struct WorkerTxInfo {
    deal_id: String,
    tx_hash: String,
}

#[marine]
pub struct TxStatusBatchResult {
    success: bool,
    error: Vec<String>,
    results: Vec<TxStatusResult>,
}

#[marine]
pub struct TxStatusResult {
    success: bool,
    error: Vec<String>,
    tx: WorkerTxInfo,
    status: String,
    block_number: Vec<String>,
}

impl TxStatusResult {
    fn ok(tx: WorkerTxInfo, status: TxStatus, block_number: Option<String>) -> Self {
        Self {
            success: true,
            error: vec![],
            status: status.to_string(),
            tx,
            block_number: block_number.map(|f| vec![f]).unwrap_or_default(),
        }
    }

    fn error(tx: WorkerTxInfo, msg: String) -> Self {
        Self {
            success: false,
            error: vec![msg],
            status: "".to_string(),
            tx,
            block_number: vec![],
        }
    }
}

enum TxStatus {
    Failed,
    Ok,
    Pending,
}

impl TxStatus {
    fn to_string(self) -> String {
        match self {
            TxStatus::Failed => "failed".to_string(),
            TxStatus::Ok => "ok".to_string(),
            TxStatus::Pending => "pending".to_string(),
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TxResp {
    status: String,
    block_number: String,
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

#[marine]
pub fn get_tx_statuses(api_endpoint: String, txs: Vec<WorkerTxInfo>) -> TxStatusBatchResult {
    //let req = TxReq::new(tx_hash).to_jsonrpc(0);
    let req_batch = txs
        .iter()
        .enumerate()
        .map(|(idx, tx)| TxReq::new(tx.tx_hash.clone()).to_jsonrpc(idx as u32))
        .collect::<Vec<_>>();

    let result: Result<Vec<JsonRpcResp<Option<TxResp>>>, _> =
        send_jsonrpc_batch(&api_endpoint, req_batch);
    match result {
        Ok(result) => {
            let results = result
                .into_iter()
                .zip(txs)
                .map(|(resp, tx)| {
                    let result: Result<_, TxError> = try {
                        if let Some(result) = resp.get_result()? {
                            let status = match result.status.as_str() {
                                "0x1" => TxStatus::Ok,
                                "0x0" => TxStatus::Failed,
                                x => Err(TxError::UnexpectedStatus(x.to_string()))?,
                            };
                            (status, Some(result.block_number))
                        } else {
                            (TxStatus::Pending, None)
                        }
                    };
                    match result {
                        Err(err) => TxStatusResult::error(tx, err.to_string()),
                        Ok((status, block_number)) => TxStatusResult::ok(tx, status, block_number),
                    }
                })
                .collect::<Vec<_>>();
            TxStatusBatchResult {
                success: true,
                error: vec![],
                results,
            }
        }
        Err(err) => TxStatusBatchResult {
            success: false,
            error: vec![err.to_string()],
            results: vec![],
        },
    }
}

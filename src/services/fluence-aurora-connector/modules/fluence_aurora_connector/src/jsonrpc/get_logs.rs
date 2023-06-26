use crate::chain::{JsonRpcReq, JsonRpcResp};
use crate::curl::send_jsonrpc;
use crate::jsonrpc::request::RequestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsReq {
    pub from_block: String,
    pub to_block: String,
    pub address: String,
    pub topics: Vec<String>,
}

impl GetLogsReq {
    pub fn to_jsonrpc(self, id: u32) -> JsonRpcReq<Vec<Self>> {
        JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            method: "eth_getLogs".to_string(),
            params: vec![self],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResp {
    // Actual data that holds all the info about the chain.
    pub data: String,
    // The block number with the chain.
    pub block_number: String,
    // true when the log was removed, due to a chain reorganization. false if its a valid log.
    pub removed: bool,
}

// pub fn get_logs(
//     url: String,
//     address: String,
//     topics: Vec<String>,
//     from_block: String,
//     to_block: String,
// ) -> Result<JsonRpcResp<Vec<GetLogsResp>>, RequestError> {
//     let req = GetLogsReq {
//         address,
//         topics,
//         from_block,
//         to_block,
//     };
//     send_jsonrpc(url, req.to_jsonrpc(0))
// }

pub fn get_logs(
    api_endpoint: String,
    address: String,
    from_block: String,
    to_block: String,
    topic: String,
) -> Result<Vec<GetLogsResp>, RequestError> {
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

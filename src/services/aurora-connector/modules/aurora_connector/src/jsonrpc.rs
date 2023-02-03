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

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcReq<T> {
    jsonrpc: String,
    id: u32,
    method: String,
    params: T,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResp {
    // Actual data that holds all the info about the deal.
    pub data: String,
    // The block number with the deal.
    pub block_number: String,
    //
    pub removed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let request = r#"
{
    "jsonrpc": "2.0",
    "method": "eth_getLogs",
    "params": [
        {
            "fromBlock": "0",
            "address": "0xD7Fa4fdaae7b69A2b7B87A860fFbDB8232310a43",
            "topics": [
                "0x04157dc3f231c23b7cbecbadb1af08b865aa2e8d6624fe39a72a17279da72278"
            ]
        }
    ],
    "id": 0
}"#;

        let result: serde_json::Result<JsonRpcReq<Vec<GetLogsReq>>> = serde_json::from_str(request);
        assert!(result.is_ok(), "cannot parse request");
        let result = result.unwrap();

        let req = GetLogsReq {
            from_block: "0".to_string(),
            address: "0xD7Fa4fdaae7b69A2b7B87A860fFbDB8232310a43".to_string(),
            topics: vec![
                "0x04157dc3f231c23b7cbecbadb1af08b865aa2e8d6624fe39a72a17279da72278".to_string(),
            ],
        };
        let jsonrpc_req = req.to_jsonrpc();

        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            serde_json::to_string(&jsonrpc_req).unwrap(),
            "jsonrpc request isn't serialized correctly"
        );
    }

    #[test]
    fn test_reponse() {
        let response = r#"
{
  "jsonrpc": "2.0",
  "id": 0,
  "result": [
    {
      "blockNumber": "0x6dc0733",
      "blockHash": "0xf17303dbff2f5d9cdc47b1deae27c75a0e48e77376527fc599022ac30e9bf84a",
      "transactionIndex": "0x0",
      "transactionHash": "0x1c5f55a859bd36f6abffef1f91da1726c9f94a64e5c09df1d61ab2425a4cd2fd",
      "logIndex": "0x2",
      "address": "0xd7fa4fdaae7b69a2b7b87a860ffbdb8232310a43",
      "topics": [
        "0x04157dc3f231c23b7cbecbadb1af08b865aa2e8d6624fe39a72a17279da72278"
      ],
      "data": "0x0123456789",
      "removed": false
    }]
}
"#;
        let result: serde_json::Result<JsonRpcResp<Vec<GetLogsResp>>> =
            serde_json::from_str(response);
        assert!(result.is_ok(), "cannot parse response");
        let result = result.unwrap();
        let result = result.get_result();
        assert!(result.is_ok(), "wrong jsonrpc method or id");
        let result = result.unwrap();
        assert_eq!(result.len(), 1, "wrong parsed number of results");

        assert!(
            !result[0].removed,
            "parsed block marked as removed when it's not"
        );
        assert_eq!(
            result[0].block_number, "0x6dc0733",
            "parsed block's number is wrong"
        );
        assert_eq!(
            result[0].data, "0x0123456789",
            "parsed block's data is wrong"
        );
    }
}

use crate::{jsonrpc::{JsonRpcReq, get_logs::GetLogsReq}, chain::log::Log};

use super::*;

#[test]
fn test_get_logs_request() {
    let request = r#"
{
    "jsonrpc": "2.0",
    "method": "eth_getLogs",
    "params": [
        {
            "fromBlock": "0",
            "toBlock": "latest",
            "address": "0xD7Fa4fdaae7b69A2b7B87A860fFbDB8232310a43",
            "topics": [
                "0x04157dc3f231c23b7cbecbadb1af08b865aa2e8d6624fe39a72a17279da72278"
            ]
        }
    ],
    "id": 0
}"#;

    let result: serde_json::Result<JsonRpcReq<GetLogsReq>> = serde_json::from_str(request);
    assert!(result.is_ok(), "cannot parse request");
    let result = result.unwrap();

    let req = GetLogsReq {
        from_block: "0".to_string(),
        to_block: "latest".to_string(),
        address: "0xD7Fa4fdaae7b69A2b7B87A860fFbDB8232310a43".to_string(),
        topics: vec![
            "0x04157dc3f231c23b7cbecbadb1af08b865aa2e8d6624fe39a72a17279da72278".to_string(),
        ],
    };
    let jsonrpc_req = req.to_jsonrpc(0);

    assert_eq!(
        serde_json::to_string(&result).unwrap(),
        serde_json::to_string(&jsonrpc_req).unwrap(),
        "jsonrpc request isn't serialized correctly"
    );
}

#[test]
fn test_get_logs_reponse() {
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
    let result: serde_json::Result<JsonRpcResp<Vec<Log>>> = serde_json::from_str(response);
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

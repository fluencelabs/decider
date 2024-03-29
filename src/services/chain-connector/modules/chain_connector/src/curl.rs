use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::request::RequestError::{CurlError, OtherError, ParseError, RpcSerializeError};
use crate::jsonrpc::{JsonRpcReq, JsonRpcResp};

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult;
}

#[rustfmt::skip]
fn curl_params(url: &str, data: &str) -> Vec<String> {
    let params = vec![
        url,
        // To avoid unnecessary data in stderr
        "--no-progress-meter",
        "-X", "POST",
        "-H", "Content-Type: application/json",
        // To avoid hanging on try to connect
        // TODO: what the best timeout?
        "--connect-timeout", "5",
        // Do not try to reconnect, just make another call
        "--retry", "0",
        "--data", data,
    ];

    params.into_iter().map(String::from).collect::<_>()
}

pub fn curl(url: &str, data: &str) -> Result<String, RequestError> {
    let params = curl_params(url, data);
    let result = curl_request(params);

    match result.into_std() {
        None => Err(OtherError(
            "curl output is not a valid UTF-8 string".to_string(),
        )),
        Some(Err(err)) => Err(CurlError(err)),
        Some(Ok(result)) => Ok(result),
    }
}

pub fn send_jsonrpc<Req: Serialize, Resp: DeserializeOwned>(
    url: &str,
    req: JsonRpcReq<Req>,
) -> Result<JsonRpcResp<Resp>, RequestError> {
    let req = serde_json::to_string(&req).map_err(RpcSerializeError)?;
    log::debug!("json rpc request: {}", req);
    let result = curl(url, &req)?;
    log::debug!("json rpc response: {}", result);

    match serde_json::from_str(&result) {
        Err(err) => Err(ParseError(err, result)),
        Ok(result) => Ok(result),
    }
}

pub fn send_jsonrpc_batch<Req: Serialize, Resp: DeserializeOwned>(
    url: &str,
    reqs: Vec<JsonRpcReq<Req>>,
) -> Result<Vec<JsonRpcResp<Resp>>, RequestError> {
    let reqs = serde_json::json!(reqs);
    log::debug!("json rpc batch request: {}", reqs);
    let req = serde_json::to_string(&reqs).map_err(RpcSerializeError)?;
    let result = curl(url, &req)?;
    log::debug!("json rpc batch response: {}", result);

    // Parse the result. Note that errors in a JSON RPC request will result in
    // a HTML in a response.
    match serde_json::from_str(&result) {
        Err(err) => Err(ParseError(err, result)),
        Ok(result) => Ok(result),
    }
}

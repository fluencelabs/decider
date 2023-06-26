use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;
use serde_json::json;
use thiserror::Error;
use url::Url;

use crate::chain::{JsonRpcReq, JsonRpcResp};
use crate::curl::{curl, send_jsonrpc, send_jsonrpc_batch};
use crate::jsonrpc::block_number::BlockNumberReq;
use crate::jsonrpc::get_logs::{GetLogsReq, GetLogsResp};
use crate::jsonrpc::*;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("`curl` returned error: {0}")]
    CurlError(String),
    #[error(
        "the response isn't in JSON RPC `eth_getLogs` reponse format: {0}. Full response: {1}"
    )]
    ParseError(serde_json::Error, String),
    #[error("error occured with `curl`: {0}")]
    OtherError(String),
    #[error("invalid URL: {0}")]
    ParseUrlError(#[source] url::ParseError),
    #[error("error serializing JsonRpc request: {0}")]
    RpcSerializeError(serde_json::Error),
}

/// Returns `Err` if `url` is not a valid URL
pub fn check_url(url: &str) -> Result<(), RequestError> {
    Url::parse(url)
        .map_err(RequestError::ParseUrlError)
        .map(|_| ())
}

use thiserror::Error;
use url::Url;

use crate::jsonrpc::JsonRpcError;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("`curl` returned error: {0}")]
    CurlError(String),
    #[error(
        "the response isn't in required JSON RPC reponse format: {0}. Full response: {1}"
    )]
    ParseError(serde_json::Error, String),
    #[error("error occured with `curl`: {0}")]
    OtherError(String),
    #[error("invalid URL: {0}")]
    ParseUrlError(#[source] url::ParseError),
    #[error("error serializing JsonRpc request: {0}")]
    RpcSerializeError(serde_json::Error),
    #[error("json rpc error: {0}")]
    JsonRpcError(#[from] JsonRpcError),
}

/// Returns `Err` if `url` is not a valid URL
pub fn check_url(url: &str) -> Result<(), RequestError> {
    Url::parse(url)
        .map_err(RequestError::ParseUrlError)
        .map(|_| ())
}

use crate::jsonrpc::*;
use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;
use serde_json::json;
use thiserror::Error;
use url::Url;

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
}

pub fn check_url(url: &str) -> Result<(), RequestError> {
    Url::parse(url).map_err(RequestError::ParseUrlError).map(|_| ())
}

pub fn get_block_number(url: String) -> Result<JsonRpcResp<String>, RequestError> {
    use RequestError::*;

    let req = json!(BlockNumberReq::to_jsonrpc());
    let req = serde_json::to_string(&req).unwrap();
    log::debug!("request: {}", req);
    // Make a request
    let result = curl_request(curl_params(url, req)).into_std();
    let result = match result {
        None => {
            return Err(OtherError(
                "curl output is not a valid UTF-8 string".to_string(),
            ));
        }
        Some(Err(err)) => return Err(CurlError(err)),
        Some(Ok(result)) => result,
    };
    match serde_json::from_str(&result) {
        Err(err) => Err(ParseError(err, result)),
        Ok(result) => Ok(result),
    }
}

pub fn get_logs_batch(
    url: String,
    reqs: Vec<JsonRpcReq<Vec<GetLogsReq>>>,
) -> Result<Vec<JsonRpcResp<Vec<GetLogsResp>>>, RequestError> {
    use RequestError::*;
    let req = json!(reqs);
    let req =
        serde_json::to_string(&req).expect("can't convert get_logs_batch JRPC request to JSON");
    log::debug!("request: {}", req);
    let result = curl_request(curl_params(url, req)).into_std();
    let result = match result {
        None => {
            return Err(OtherError(
                "curl output is not a valid UTF-8 string".to_string(),
            ));
        }
        Some(Err(err)) => return Err(CurlError(err)),
        Some(Ok(result)) => result,
    };
    // Parse the result. Note that errors in a JSON RPC request will result in
    // a HTML in a response.
    match serde_json::from_str(&result) {
        Err(err) => Err(ParseError(err, result)),
        Ok(result) => Ok(result),
    }
}

pub fn get_logs(
    url: String,
    address: String,
    topics: Vec<String>,
    from_block: String,
    to_block: String,
) -> Result<JsonRpcResp<Vec<GetLogsResp>>, RequestError> {
    use RequestError::*;

    // Create a JSON RPC request
    let req = GetLogsReq {
        address,
        topics,
        from_block,
        to_block,
    };
    let req = json!(req.to_jsonrpc(0));
    let req = serde_json::to_string(&req).expect("can't convert get_logs JRPC request to JSON");
    log::debug!("request: {}", req);

    // Make a request
    let result = curl_request(curl_params(url, req)).into_std();
    let result = match result {
        None => {
            return Err(OtherError(
                "curl output is not a valid UTF-8 string".to_string(),
            ));
        }
        Some(Err(err)) => return Err(CurlError(err)),
        Some(Ok(result)) => result,
    };
    // Parse the result. Note that errors in a JSON RPC request will result in
    // a HTML in a response.
    match serde_json::from_str(&result) {
        Err(err) => Err(ParseError(err, result)),
        Ok(result) => Ok(result),
    }
}

#[rustfmt::skip]
fn curl_params(url: String, data: String) -> Vec<String> {
    let params = vec![
        url.as_str(),
        // To avoid unnecessary data in stderr
        "--no-progress-meter",
        "-X", "POST",
        "-H", "Content-Type: application/json",
        // To avoid hanging on try to connect
        // TODO: what the best timeout?
        "--connect-timeout", "5",
        // Do not try to reconnect, just make another call
        "--retry", "0",
        "--data", data.as_str(),
    ];

    params.into_iter().map(String::from).collect::<_>()
}

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult;
}

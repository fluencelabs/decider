use crate::chain::chain_data::{parse_chain_data, ChainDataError};
use crate::curl::send_jsonrpc;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::{JsonRpcError, JsonRpcReq, JsonRpcResp};
use ethabi::ParamType::FixedBytes;
use ethabi::{Function, ParamType, StateMutability, Token};
use marine_rs_sdk::marine;
use serde_json::json;
use thiserror::Error;

#[derive(Debug)]
pub enum DealStatus {
    INACTIVE,
    ACTIVE,
    ENDED,
}

fn function() -> Function {
    #[allow(deprecated)]
    Function {
        name: "getStatus".to_owned(),
        inputs: vec![],
        outputs: vec![],
        constant: None,
        state_mutability: StateMutability::View,
    }
}

fn signature() -> ParamType {
    FixedBytes(32)
}

#[derive(Debug, Error)]
pub enum DealStatusError {
    #[error(transparent)]
    ReceiveRPC(#[from] JsonRpcError),
    #[error(transparent)]
    SendRPC(#[from] RequestError),
    #[error(transparent)]
    ChainData(#[from] ChainDataError),
    #[error("got unexpected status from the chain `{result}`")]
    UnknownStatus { result: String },
}

pub fn decode_status(data: &str) -> Result<DealStatus, DealStatusError> {
    let tokens = parse_chain_data(data, &[signature()])?;
    log::warn!("RECEIVED TOKENS: {:?}", tokens);
    Ok(DealStatus::INACTIVE)
}

#[marine]
pub fn get_status(deal_id: &str, api_endpoint: &str) {
    let res: Result<DealStatus, DealStatusError> = try {
        let function = function();
        let bytes = function.encode_input(&[]).unwrap();
        let input = format!("0x{}", hex::encode(bytes));
        let req = JsonRpcReq {
            jsonrpc: "2.0".to_owned(),
            id: 0,
            method: "eth_call".to_owned(),
            params: vec![json!({"data": input, "to": deal_id})],
        };
        let response: JsonRpcResp<String> = send_jsonrpc(api_endpoint, req)?;
        let status = response.get_result()?;
        decode_status(&status)?
    };
    log::warn!("RESULT: {:?}", res);
}

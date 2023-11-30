use crate::chain::chain_data::{parse_chain_data, ChainDataError};
use crate::curl::send_jsonrpc;
use crate::jsonrpc::deal_status::DealStatusError::UnknownStatus;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::{JsonRpcError, JsonRpcReq, JsonRpcResp};
use ethabi::ParamType::FixedBytes;
use ethabi::{Function, ParamType, StateMutability};
use marine_rs_sdk::marine;
use serde_json::json;
use thiserror::Error;

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

#[derive(Debug)]
pub enum DealStatus {
    INACTIVE = 0,
    ACTIVE,
    ENDED,
}

impl DealStatus {
    fn from(num: u8) -> Option<Self> {
        match num {
            0 => Some(DealStatus::INACTIVE),
            1 => Some(DealStatus::ACTIVE),
            2 => Some(DealStatus::ENDED),
            _ => None,
        }
    }
}

impl ToString for DealStatus {
    fn to_string(&self) -> String {
        match self {
            DealStatus::INACTIVE => "INACTIVE".to_string(),
            DealStatus::ACTIVE => "ACTIVE".to_string(),
            DealStatus::ENDED => "ENDED".to_string(),
        }
    }
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

pub fn decode_status(data: &str) -> Result<DealStatus, DealStatusError> {
    let mut tokens = parse_chain_data(data, &[signature()])?;
    let token = tokens.pop().ok_or(UnknownStatus {
        result: data.to_owned(),
    })?;
    let status_num: Vec<u8> = token.into_fixed_bytes().ok_or(UnknownStatus {
        result: data.to_owned(),
    })?;
    let status_num = status_num.last().ok_or(UnknownStatus {
        result: data.to_owned(),
    })?;
    DealStatus::from(*status_num).ok_or(UnknownStatus {
        result: data.to_owned(),
    })
}

#[marine]
pub struct DealStatusResult {
    pub status: String,
    pub success: bool,
    pub error: Vec<String>,
}

impl DealStatusResult {
    pub fn ok(status: DealStatus) -> Self {
        Self {
            status: status.to_string(),
            success: true,
            error: vec![],
        }
    }

    pub fn error(error: DealStatusError) -> Self {
        Self {
            status: "".to_string(),
            success: false,
            error: vec![error.to_string()],
        }
    }
}

#[marine]
pub fn get_status(deal_id: &str, api_endpoint: &str) -> DealStatusResult {
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
    match res {
        Ok(status) => DealStatusResult::ok(status),
        Err(e) => DealStatusResult::error(e),
    }
}

#[cfg(test)]
mod tests {
    use super::JsonRpcReq;
    use marine_rs_sdk_test::marine_test;
    use serde::Deserialize;
    use std::sync::{Arc, Mutex};

    #[derive(Deserialize, Debug)]
    struct DealStatusRequest {
        data: String,
        to: String, // deal_id
    }

    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn test_get_status(connector: marine_test_env::chain_connector::ModuleInterface) {
        let mut server = mockito::Server::new();
        let url = server.url();
        const DEAL_ID: &'static str = "0x6328bb918a01603adc91eae689b848a9ecaef26d";
        let jsonrpc_inactive = r#"{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000000"}"#;
        let jsonrpc_active = r#"{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000001"}"#;
        let jsonrpc_ended = r#"{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000002"}"#;
        let jsonrpc_unknown = r#"{"jsonrpc":"2.0","id":0,"result":"0x"}"#;
        let jsonrpcs = Arc::new(Mutex::new(vec![
            jsonrpc_unknown,
            jsonrpc_ended,
            jsonrpc_active,
            jsonrpc_inactive,
        ]));
        let mock = server
            .mock("POST", "/")
            .with_body_from_request(move |req| {
                let body = req.body().expect("mock: get req body");
                let body: JsonRpcReq<DealStatusRequest> =
                    serde_json::from_slice(body).expect("mock: parse req body as json");
                assert_eq!(body.params[0].to, DEAL_ID);

                let jsonrpc = jsonrpcs.lock().unwrap().pop().unwrap();
                jsonrpc.into()
            })
            .expect(4)
            .with_status(200)
            .with_header("content-type", "application/json")
            .create();

        let invalid_mock = server
            .mock("POST", "/")
            .expect(0)
            .with_status(404)
            .with_body("invalid mock was hit. Check that request body matches 'match_body' clause'")
            .create();

        let result: marine_test_env::chain_connector::DealStatusResult =
            connector.get_status(DEAL_ID.to_string(), url.clone());
        assert!(result.success);
        assert!(result.error.is_empty());
        assert_eq!(result.status, "INACTIVE");

        let result: marine_test_env::chain_connector::DealStatusResult =
            connector.get_status(DEAL_ID.to_string(), url.clone());
        assert!(result.success);
        assert!(result.error.is_empty());
        assert_eq!(result.status, "ACTIVE");

        let result: marine_test_env::chain_connector::DealStatusResult =
            connector.get_status(DEAL_ID.to_string(), url.clone());
        assert!(result.success);
        assert!(result.error.is_empty());
        assert_eq!(result.status, "ENDED");

        let result: marine_test_env::chain_connector::DealStatusResult =
            connector.get_status(DEAL_ID.to_string(), url);
        assert!(!result.success);
        assert!(!result.error.is_empty());

        invalid_mock.assert();
        mock.assert();
    }
}

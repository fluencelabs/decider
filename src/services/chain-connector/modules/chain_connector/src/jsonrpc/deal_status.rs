use std::fmt::Display;
use crate::chain::chain_data::{parse_chain_data, ChainDataError};
use crate::curl::send_jsonrpc_batch;
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
    InsufficientFunds = 0,
    Active,
    Ended,
    NotEnoughWorkers
}

impl DealStatus {
    fn from(num: u8) -> Option<Self> {
        match num {
            0 => Some(DealStatus::InsufficientFunds),
            1 => Some(DealStatus::Active),
            2 => Some(DealStatus::Ended),
            3 => Some(DealStatus::NotEnoughWorkers),
            _ => None,
        }
    }
}

impl Display for DealStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DealStatus::InsufficientFunds => "INSUFFICIENT_FUNDS".to_string(),
            DealStatus::Active => "ACTIVE".to_string(),
            DealStatus::Ended => "ENDED".to_string(),
            DealStatus::NotEnoughWorkers => "NOT_ENOUGH_WORKERS".to_string(),
        };
        write!(f, "{}", str)
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
    pub deal_id: String,
    pub success: bool,
    pub error: Vec<String>,
}

impl DealStatusResult {
    pub fn ok(deal_id: String, status: DealStatus) -> Self {
        Self {
            status: status.to_string(),
            deal_id,
            success: true,
            error: vec![],
        }
    }

    pub fn error(deal_id: String, error: DealStatusError) -> Self {
        Self {
            status: "".to_string(),
            deal_id,
            success: false,
            error: vec![error.to_string()],
        }
    }
}

#[marine]
pub struct DealStatusBatchResult {
    pub statuses: Vec<DealStatusResult>,
    pub success: bool,
    pub error: Vec<String>,
}

impl DealStatusBatchResult {
    pub fn ok(statuses: Vec<DealStatusResult>) -> Self {
        Self {
            statuses,
            success: true,
            error: vec![],
        }
    }

    pub fn error(error: DealStatusError) -> Self {
        Self {
            statuses: vec![],
            success: false,
            error: vec![error.to_string()],
        }
    }
}

#[marine]
pub fn get_status_batch(api_endpoint: &str, deal_ids: Vec<String>) -> DealStatusBatchResult {
    let res: Result<_, DealStatusError> = try {
        let function = function();
        let bytes = function.encode_input(&[]).unwrap();
        let input = format!("0x{}", hex::encode(bytes));
        let batch = deal_ids
            .iter()
            .enumerate()
            .map(|(idx, deal_id)| JsonRpcReq {
                jsonrpc: "2.0".to_owned(),
                id: idx as u32,
                method: "eth_call".to_owned(),
                // TODO: is there a situation where we should ask not for 'latest', but for a specific block?
                params: vec![json!({"data": input, "to": deal_id}), json!("latest")],
            })
            .collect::<_>();
        let mut responses: Vec<JsonRpcResp<String>> = send_jsonrpc_batch(api_endpoint, batch)?;
        // Sort by id to match the order of the original requests
        responses.sort_by(|a, b| a.id.cmp(&b.id));
        responses
            .into_iter()
            .zip(deal_ids)
            .map(|(result, deal_id)| {
                let result = try {
                    let result = result.get_result()?;
                    decode_status(&result)?
                };
                match result {
                    Ok(status) => DealStatusResult::ok(deal_id, status),
                    Err(e) => DealStatusResult::error(deal_id, e),
                }
            })
            .collect::<_>()
    };
    match res {
        Ok(statuses) => DealStatusBatchResult::ok(statuses),
        Err(e) => DealStatusBatchResult::error(e),
    }
}

#[cfg(test)]
mod tests {
    use super::JsonRpcReq;
    use marine_rs_sdk_test::marine_test;
    use serde::Deserialize;
    use serde_json::Value;
    use std::sync::{Arc, Mutex};

    #[derive(Deserialize, Debug)]
    struct DealStatusRequest {
        data: String,
        to: String, // deal_id
    }

    #[derive(Deserialize, Debug)]
    struct DealStatusParams {
        data: String,
        to: String, // deal_id
    }


    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn test_get_status(connector: marine_test_env::chain_connector::ModuleInterface) {
        let mut server = mockito::Server::new();
        let url = server.url();
        const DEAL_ID: &'static str = "0x6328bb918a01603adc91eae689b848a9ecaef26d";
        const DEAL_ID_2: &'static str = "0x6328bb918a01603adc91eae689b848a9ecaef26f";

        let jsonrpc_INSUFFICIENT_FUNDS = r#"[{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000000"}]"#;
        let jsonrpc_active = r#"[{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000001"}]"#;
        let jsonrpc_ended = r#"[{"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000002"}]"#;
        let jsonrpc_unknown = r#"[{"jsonrpc":"2.0","id":0,"result":"0x"}]"#;
        let jsonrpc_2 = r#"[
            {"jsonrpc":"2.0","id":0,"result":"0x0000000000000000000000000000000000000000000000000000000000000000"}, 
            {"jsonrpc":"2.0","id":0,"result":"0x"} 
        ]"#;

        let jsonrpcs = Arc::new(Mutex::new(vec![
            jsonrpc_2,
            jsonrpc_unknown,
            jsonrpc_ended,
            jsonrpc_active,
            jsonrpc_INSUFFICIENT_FUNDS,
        ]));
        let mock = server
            .mock("POST", "/")
            .with_body_from_request(move |req| {
                let body = req.body().expect("mock: get req body");
                let body: Vec<JsonRpcReq<Value>> =
                    serde_json::from_slice(body).expect("mock: parse req body as json");
                assert!(!body.is_empty());
                let block = serde_json::from_value::<String>(body[0].params[1].clone())
                    .expect("mock: parse deal status request");
                assert_eq!(block, "latest");
                let addr = serde_json::from_value::<DealStatusRequest>(body[0].params[0].clone())
                    .expect("mock: parse deal status request");
                assert_eq!(addr.to, DEAL_ID);
                if body.len() == 2 {
                    let addr = serde_json::from_value::<DealStatusRequest>(body[1].params[0].clone())
                        .expect("mock: parse deal status request");
                    assert_eq!(addr.to, DEAL_ID_2)
                }

                let jsonrpc = jsonrpcs.lock().unwrap().pop().unwrap();
                jsonrpc.into()
            })
            .expect(5)
            .with_status(200)
            .with_header("content-type", "application/json")
            .create();

        let invalid_mock = server
            .mock("POST", "/")
            .expect(0)
            .with_status(404)
            .with_body("invalid mock was hit. Check that request body matches 'match_body' clause'")
            .create();

        let result = connector.get_status_batch(url.clone(), vec![DEAL_ID.to_string()]);
        assert!(result.success, "error: {}", result.error[0]);
        assert!(result.error.is_empty());

        assert!(!result.statuses.is_empty());
        assert!(
            result.statuses[0].success,
            "error: {}",
            result.statuses[0].error[0]
        );
        assert!(result.statuses[0].error.is_empty());

        assert_eq!(result.statuses[0].status, "INSUFFICIENT_FUNDS");
        assert_eq!(result.statuses[0].deal_id, DEAL_ID);

        let result = connector.get_status_batch(url.clone(), vec![DEAL_ID.to_string()]);
        assert!(result.success, "error: {}", result.error[0]);
        assert!(result.error.is_empty());
        assert!(!result.statuses.is_empty());
        assert!(
            result.statuses[0].success,
            "error: {}",
            result.statuses[0].error[0]
        );
        assert!(result.statuses[0].error.is_empty());
        assert_eq!(result.statuses[0].status, "ACTIVE");
        assert_eq!(result.statuses[0].deal_id, DEAL_ID);

        let result = connector.get_status_batch(url.clone(), vec![DEAL_ID.to_string()]);
        assert!(result.success, "error: {}", result.error[0]);
        assert!(result.error.is_empty());
        assert!(!result.statuses.is_empty());
        assert!(
            result.statuses[0].success,
            "error: {}",
            result.statuses[0].error[0]
        );

        assert!(result.statuses[0].error.is_empty());
        assert_eq!(result.statuses[0].status, "ENDED");
        assert_eq!(result.statuses[0].deal_id, DEAL_ID);

        let result = connector.get_status_batch(url.clone(), vec![DEAL_ID.to_string()]);
        assert!(result.success, "error: {}", result.error[0]);
        assert!(result.error.is_empty());
        assert!(!result.statuses.is_empty());
        assert!(!result.statuses[0].success);
        assert!(!result.statuses[0].error.is_empty());
        assert_eq!(result.statuses[0].deal_id, DEAL_ID);

        let result =
            connector.get_status_batch(url, vec![DEAL_ID.to_string(), DEAL_ID_2.to_string()]);
        assert!(result.success, "error: {}", result.error[0]);
        assert!(result.error.is_empty());
        assert_eq!(result.statuses.len(), 2);
        assert!(
            result.statuses[0].success,
            "error: {}",
            result.statuses[0].error[0]
        );

        assert!(result.statuses[0].error.is_empty());
        assert_eq!(result.statuses[0].deal_id, DEAL_ID);
        assert_eq!(result.statuses[0].status, "INSUFFICIENT_FUNDS");

        assert!(!result.statuses[1].success,);
        assert!(!result.statuses[1].error.is_empty());
        assert_eq!(result.statuses[1].deal_id, DEAL_ID_2);

        invalid_mock.assert();
        mock.assert();
    }
}

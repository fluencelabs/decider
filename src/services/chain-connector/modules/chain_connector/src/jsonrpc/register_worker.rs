use std::str::FromStr;

use ethabi::{Function, Param, ParamType, StateMutability, Token};
use libp2p_identity::{ParseError, PeerId};
use marine_rs_sdk::marine;
use thiserror::Error;

use crate::jsonrpc::register_worker::EncodeRegisterWorkerError::InvalidWorkerId;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::JsonRpcError;
use crate::peer_id::serialize_peer_id;

const GAS_MULTIPLIER: f64 = 0.20;

#[derive(Debug, Error)]
pub enum EncodeRegisterWorkerError {
    #[error("invalid worker id {1}: {0:?}")]
    InvalidWorkerId(#[source] ParseError, &'static str),
    #[error("error encoding function inputs: {0:?}")]
    EncodeInput(#[from] ethabi::Error),
    #[error("invalid deal contract addr: {0:?}")]
    ParseDealAddr(#[source] clarity::Error),
    #[error("invalid private key")]
    InvalidPrivateKey(#[source] Box<dyn std::error::Error>),
    #[error("error sending transaction to rpc: {0:?}")]
    SendTransaction(#[from] RequestError),
    #[error("error sending json rpc: {0}")]
    JsonRpcError(#[from] JsonRpcError),
}

#[marine]
pub struct EncodeRegisterWorkerResult {
    success: bool,
    data: Vec<u8>,
    error: Vec<String>,
}

impl EncodeRegisterWorkerResult {
    fn ok(data: Vec<u8>) -> Self {
        Self {
            success: true,
            data,
            error: vec![],
        }
    }

    fn error(err: EncodeRegisterWorkerError) -> Self {
        Self {
            success: false,
            data: vec![],
            error: vec![err.to_string()],
        }
    }
}

#[marine]
pub fn encode_register_worker(
    unit_id: Vec<u8>,
    worker_id: &str,
) -> EncodeRegisterWorkerResult {
    let r: Result<_, EncodeRegisterWorkerError> = try {
        let data = encode_call(unit_id.clone(), worker_id)?;

        data
    };

    match r {
        Ok(data) => EncodeRegisterWorkerResult::ok(data),
        Err(err) => EncodeRegisterWorkerResult::error(err),
    }
}

/// Description of the `setWorker` function from the `chain.workers` smart contract on chain
fn function() -> Function {
    #[allow(deprecated)]
    Function {
        name: String::from("setWorker"),
        inputs: vec![
            Param {
                name: String::from("patId"),
                kind: ParamType::FixedBytes(32),
                internal_type: None,
            },
            Param {
                name: String::from("workerId"),
                kind: ParamType::FixedBytes(32),
                internal_type: None,
            },
        ],
        outputs: vec![],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    }
}

/// Encode `setWorker` call to bytes
fn encode_call(unit_id: Vec<u8>, worker_id: &str) -> Result<Vec<u8>, EncodeRegisterWorkerError> {
    // let unit_id = decode_hex(unit_id).map_err(|e| EncodeArgument(e, "unit_id"))?;
    let unit_id = Token::FixedBytes(unit_id);

    let worker_id = PeerId::from_str(worker_id).map_err(|e| InvalidWorkerId(e, "worker_id"))?;
    let worker_id = serialize_peer_id(worker_id);
    let worker_id = Token::FixedBytes(worker_id);

    log::debug!("unit_id {unit_id}; worker_id {worker_id}");
    let input = function().encode_input(&[unit_id, worker_id])?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    use marine_rs_sdk_test::CallParameters;

    use crate::hex::decode_hex;
    use crate::jsonrpc::register_worker::{encode_call, function};

    fn unit_id() -> Vec<u8> {
        decode_hex("0xe532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e")
            .expect("decode pat id from hex")
    }
    const WORKER_ID: &str = "12D3KooWFNriiBhsSogV5dxdM3zzTiZwUTt6eoDpMys781qB6WyA";
    const WORKERS: &str = "0x908aEBfb6051Bca6d1e684586d7760e53C4c736C";
    const PRIVATE_KEY: &str = "0xbb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a";

    #[test]
    fn gen_call() {
        // function setWorker(bytes32 patId, bytes32 workerId) external
        let f = function();
        assert_eq!(f.signature(), "setWorker(bytes32,bytes32)");
        let signature_hex = hex::encode(f.short_signature());
        assert_eq!(signature_hex, "d5053ab0");

        let input = encode_call(unit_id(), WORKER_ID).expect("encode call");
        let input = hex::encode(input);

        assert_eq!(input, "d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5");
    }

    // Set env RUST_LOGGER="mockito=debug" to enable Mockito's logs
    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn register(connector: marine_test_env::chain_connector::ModuleInterface) {

        let cp = CallParameters {
            init_peer_id: "".to_string(),
            service_id: "".to_string(),
            service_creator_peer_id: "".to_string(),
            host_id: "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE".to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };
        let result = connector.encode_register_worker_cp(
            unit_id().into(),
            WORKER_ID.into(),
            cp,
        );
        assert!(
            result.success,
            "error in encode_register_worker: {}",
            result.error[0]
        );
        assert_eq!(hex::encode(result.data), "d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5");
    }
}

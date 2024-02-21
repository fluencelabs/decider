use std::convert::TryInto;
use std::num::ParseIntError;
use std::str::FromStr;

use clarity::{Address, PrivateKey, Transaction};
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use libp2p_identity::{ParseError, PeerId};
use marine_rs_sdk::marine;
use thiserror::Error;

use crate::chain::chain_info::ChainInfo;
use crate::curl::send_jsonrpc;
use crate::hex::{decode_hex, u128_from_hex};
use crate::jsonrpc::register_worker::RegisterWorkerError::{
    InvalidPrivateKey, InvalidWorkerId, ParseDealAddr,
};
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::{JsonRpcError, JsonRpcReq, JSON_RPC_VERSION};
use crate::peer_id::serialize_peer_id;

const GAS_MULTIPLIER: f64 = 0.20;

#[derive(Debug, Error)]
pub enum RegisterWorkerError {
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
    #[error("error parsing eth_getTransactionCount response: {0}")]
    InvalidTxCount(#[from] ParseIntError),
}

#[marine]
pub struct RegisterWorkerResult {
    success: bool,
    tx_hash: Vec<String>,
    error: Vec<String>,
}

impl RegisterWorkerResult {
    fn ok(tx_hash: String) -> Self {
        Self {
            success: true,
            tx_hash: vec![tx_hash],
            error: vec![],
        }
    }

    fn error(err: RegisterWorkerError) -> Self {
        Self {
            success: false,
            tx_hash: vec![],
            error: vec![err.to_string()],
        }
    }
}

#[marine]
pub fn register_worker(
    unit_id: Vec<u8>,
    worker_id: &str,
    chain: ChainInfo,
    deal_addr: &str,
) -> RegisterWorkerResult {
    let endpoint = &chain.api_endpoint;
    let gas = chain.workers_gas;
    let network_id = chain.network_id;

    let r: Result<_, RegisterWorkerError> = try {
        let key = parse_wallet_key(&chain.wallet_key)?;
        let input = encode_call(unit_id.clone(), worker_id)?;
        let nonce = load_nonce(key.to_address(), endpoint)?;
        let gas_price = get_gas_price(endpoint)?;
        let tx = make_tx(input, key, gas, nonce, gas_price, deal_addr, network_id)?;
        log::debug!(
            "wallet {}; wallet address {}; unit_id {}; worker_id {worker_id}; nonce {nonce}; gas_price {gas_price}", 
            chain.wallet_key, key.to_address(), hex::encode(unit_id)
        );
        log::debug!("tx {tx}");
        send_tx(tx, endpoint)?
    };

    match r {
        Ok(tx_hash) => RegisterWorkerResult::ok(tx_hash),
        Err(err) => RegisterWorkerResult::error(err),
    }
}

/// Send transaction to RPC
fn send_tx(tx: String, api_endpoint: &str) -> Result<String, RegisterWorkerError> {
    let req = JsonRpcReq {
        id: 0,
        jsonrpc: JSON_RPC_VERSION.to_string(),
        method: "eth_sendRawTransaction".to_string(),
        params: vec![tx],
    };
    let response = send_jsonrpc(api_endpoint, req)?;
    let tx_id: String = response.get_result()?;

    Ok(tx_id)
}

/// Load nonce from RPC
fn load_nonce(address: Address, api_endpoint: &str) -> Result<u128, RegisterWorkerError> {
    // '{"method":"eth_getTransactionCount","params":["0x8D97689C9818892B700e27F316cc3E41e17fBeb9", "latest"],"id":1,"jsonrpc":"2.0"}'
    let req = JsonRpcReq {
        id: 0,
        jsonrpc: JSON_RPC_VERSION.to_string(),
        method: "eth_getTransactionCount".to_string(),
        params: vec![address.to_string(), "pending".into()],
    };
    let response = send_jsonrpc(api_endpoint, req)?;
    let count_hex: String = response.get_result()?;
    let count = u128_from_hex(&count_hex)?;

    Ok(count)
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
fn encode_call(unit_id: Vec<u8>, worker_id: &str) -> Result<Vec<u8>, RegisterWorkerError> {
    // let unit_id = decode_hex(unit_id).map_err(|e| EncodeArgument(e, "unit_id"))?;
    let unit_id = Token::FixedBytes(unit_id);

    let worker_id = PeerId::from_str(worker_id).map_err(|e| InvalidWorkerId(e, "worker_id"))?;
    let worker_id = serialize_peer_id(worker_id);
    let worker_id = Token::FixedBytes(worker_id);

    log::debug!("unit_id {unit_id}; worker_id {worker_id}");
    let input = function().encode_input(&[unit_id, worker_id])?;
    Ok(input)
}

/// Load gas price from RPC
fn get_gas_price(api_endpoint: &str) -> Result<u128, RegisterWorkerError> {
    // {"jsonrpc":"2.0","id":0,"method":"eth_gasPrice","params":[]}
    // {"jsonrpc":"2.0","id":0,"result":"0x3b9aca07"}
    let req = JsonRpcReq::<()> {
        id: 0,
        jsonrpc: JSON_RPC_VERSION.to_string(),
        method: "eth_gasPrice".to_string(),
        params: vec![],
    };
    let response = send_jsonrpc(api_endpoint, req)?;
    let price: String = response.get_result()?;
    let price = u128_from_hex(&price)?;

    // increase price by GAS_MULTIPLIER so transaction are included faster
    let increase = (price as f64 * GAS_MULTIPLIER) as u128;
    let price = price.checked_add(increase).unwrap_or(price);

    Ok(price)
}

fn pk_err<E: std::error::Error + 'static>(err: E) -> RegisterWorkerError {
    InvalidPrivateKey(Box::new(err))
}

#[derive(Debug, Error)]
#[error("invalid private key size, expected 32 bytes")]
struct InvalidPrivateKeySize;

fn parse_wallet_key(wallet_key: &str) -> Result<PrivateKey, RegisterWorkerError> {
    use InvalidPrivateKeySize as PKErr;

    let private_key = decode_hex(wallet_key).map_err(pk_err)?;
    let private_key = private_key.try_into().map_err(|_| pk_err(PKErr))?;
    let private_key = PrivateKey::from_bytes(private_key).map_err(pk_err)?;

    Ok(private_key)
}

fn make_tx(
    input: Vec<u8>,
    wallet_key: PrivateKey,
    workers_gas: u64,
    nonce: u128,
    gas_price: u128,
    deal_addr: &str,
    network_id: u64,
) -> Result<String, RegisterWorkerError> {
    let workers_address = deal_addr.parse().map_err(ParseDealAddr)?;

    // Create a new transaction
    let tx = Transaction::Legacy {
        nonce: nonce.into(),
        gas_price: gas_price.into(),
        gas_limit: workers_gas.into(),
        to: workers_address,
        value: 0u32.into(),
        data: input,
        signature: None, // Not signed. Yet.
    };

    let tx = tx.sign(&wallet_key, Some(network_id)).to_bytes();
    let tx = hex::encode(tx);

    Ok(format!("0x{}", tx))
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
        let _ = ::env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .filter_module("mockito", log::LevelFilter::Debug)
            .filter_module("chain_connector", log::LevelFilter::Debug)
            .filter_module("marine_core", log::LevelFilter::Debug)
            .filter_module("wasmer_interface_types_fl", log::LevelFilter::Off)
            .is_test(true)
            .try_init();

        let get_nonce_response = r#"{"jsonrpc":"2.0","id":0,"result":"0x20"}"#;
        let send_tx_response = r#"
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": "0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05"
        }
        "#;
        let gas_price_response = r#"{"jsonrpc":"2.0","id":0,"result":"0x3b9aca07"}"#;

        // Create a mock
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/")
            .with_body_from_request(move |req| {
                let body = req.body().expect("mock: get request body");
                let body: serde_json::Value =
                    serde_json::from_slice(body).expect("mock: parse request body");
                let method = body.get("method").expect("get method");
                let method = method.as_str().expect("as str").trim_matches(|c| c == '\"');

                match method {
                    "eth_getTransactionCount" => get_nonce_response.into(),
                    "eth_sendRawTransaction" => send_tx_response.into(),
                    "eth_gasPrice" => gas_price_response.into(),
                    method => format!("'{}' not supported", method).into(),
                }
            })
            // expect exactly 3 POST requests
            .expect(3)
            .with_status(200)
            .with_header("content-type", "application/json")
            .create();

        let invalid_mock = server
            .mock("POST", "/")
            .expect(0)
            .with_status(404)
            .with_body("invalid mock was hit. Check that request body matches 'match_body' clause'")
            .create();

        let chain = marine_test_env::chain_connector::ChainInfo {
            api_endpoint: url,
            matcher: "0x6328bb918a01603adc91eae689b848a9ecaef26d".into(),
            workers_gas: 210_000,
            wallet_key: PRIVATE_KEY.into(),
            network_id: 80001,
        };
        let cp = CallParameters {
            init_peer_id: "".to_string(),
            service_id: "".to_string(),
            service_creator_peer_id: "".to_string(),
            host_id: "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE".to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };
        let result = connector.register_worker_cp(
            unit_id().into(),
            WORKER_ID.into(),
            chain,
            "0x6328bb918a01603adc91eae689b848a9ecaef26d".into(),
            cp,
        );
        assert!(
            result.success,
            "error in register_worker: {}",
            result.error[0]
        );

        // assert that there was no invalid requests
        invalid_mock.assert();

        // TODO: how to check request body?
        // check that mock was called
        mock.assert();
    }
}

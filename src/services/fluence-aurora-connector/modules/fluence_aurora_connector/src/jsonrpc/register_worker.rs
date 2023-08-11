use std::convert::TryInto;
use std::str::FromStr;

use clarity::{PrivateKey, Transaction};
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use libp2p_identity::{ParseError, PeerId};
use marine_rs_sdk::marine;
use thiserror::Error;

use crate::chain::chain_info::ChainInfo;
use crate::chain::deal_matched::PEER_ID_PREFIX;
use crate::curl::send_jsonrpc;
use crate::hex::decode_hex;
use crate::jsonrpc::register_worker::RegisterWorkerError::{
    InvalidPrivateKey, InvalidWorkerId, ParseWorkersAddr,
};
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::{JsonRpcError, JsonRpcReq, JSON_RPC_VERSION};

#[derive(Debug, Error)]
pub enum RegisterWorkerError {
    #[error("invalid worker id {1}: {0:?}")]
    InvalidWorkerId(#[source] ParseError, &'static str),
    #[error("error encoding function inputs: {0:?}")]
    EncodeInput(#[from] ethabi::Error),
    #[error("invalid workers addr: {0:?}")]
    ParseWorkersAddr(#[source] clarity::Error),
    #[error("invalid private key")]
    InvalidPrivateKey(#[source] Box<dyn std::error::Error>),
    #[error("error sending transaction to rpc: {0:?}")]
    SendTransaction(#[from] RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] JsonRpcError),
}

#[marine]
pub fn register_worker(pat_id: Vec<u8>, worker_id: &str, chain: ChainInfo) -> Vec<String> {
    // get network id from rpc
    // get gas price from rpc
    // form tx
    // sign
    // send tx to rpc
    let r: Result<_, RegisterWorkerError> = try {
        let input = encode_call(pat_id, worker_id)?;
        let nonce = load_nonce()?;
        let gas_price = get_gas_price()?;
        let endpoint = chain.api_endpoint.clone();
        let tx = make_tx(input, chain, nonce, gas_price)?;
        send_tx(tx, endpoint)?
    };

    match r {
        Ok(_) => vec![],
        Err(err) => vec![format!("{}", err)],
    }
}

// '{"jsonrpc":"2.0","id":0,"method":"eth_sendRawTransaction","params":["0xf8a802830f42408303345094908aebfb6051bca6d1e684586d7760e53c4c736c80b844d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b51ca0ff9912eec4a93c6a4591255bcd354a698fa05ba052519b0a6c15ccb8bd0ef2a8a072b700c1f4319b4046060c466e06b1d2af98c4d1dae06b01e9a6ee5a14a01f09"]}' http://127.0.0.1:8545
// {"jsonrpc":"2.0","id":0,"result":"0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05"}%

/// Send transaction to RPC
fn send_tx(tx: String, api_endpoint: String) -> Result<String, RegisterWorkerError> {
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

/// Load nonce from KV
fn load_nonce() -> Result<u128, RegisterWorkerError> {
    Ok(0)
}

/// Increment nonce in KV
fn increment_nonce(_nonce: u128) -> Result<(), RegisterWorkerError> {
    Ok(())
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
fn encode_call(pat_id: Vec<u8>, worker_id: &str) -> Result<Vec<u8>, RegisterWorkerError> {
    // let pat_id = decode_hex(pat_id).map_err(|e| EncodeArgument(e, "pat_id"))?;
    let pat_id = Token::FixedBytes(pat_id);

    let worker_id = PeerId::from_str(worker_id).map_err(|e| InvalidWorkerId(e, "worker_id"))?;
    let worker_id = worker_id.to_bytes();
    let worker_id = worker_id.into_iter().skip(PEER_ID_PREFIX.len()).collect();
    let worker_id = Token::FixedBytes(worker_id);

    let input = function().encode_input(&[pat_id, worker_id])?;
    Ok(input)
}

/// Load gas price from RPC
fn get_gas_price() -> Result<u128, RegisterWorkerError> {
    Ok(1_000_000)
}

fn pk_err<E: std::error::Error + 'static>(err: E) -> RegisterWorkerError {
    InvalidPrivateKey(Box::new(err))
}

#[derive(Debug, Error)]
#[error("invalid private key size, expected 32 bytes")]
struct InvalidPrivateKeySize;

fn make_tx(
    input: Vec<u8>,
    chain: ChainInfo,
    nonce: u128,
    gas_price: u128,
) -> Result<String, RegisterWorkerError> {
    use InvalidPrivateKeySize as PKErr;

    let private_key = decode_hex(&chain.wallet_key).map_err(pk_err)?;
    let private_key = private_key.try_into().map_err(|_| pk_err(PKErr))?;
    let private_key = PrivateKey::from_bytes(private_key).map_err(pk_err)?;

    let workers_address = chain.workers.parse().map_err(ParseWorkersAddr)?;

    // Create a new transaction
    let tx = Transaction::Legacy {
        nonce: nonce.into(),
        gas_price: gas_price.into(),
        gas_limit: chain.workers_gas.into(),
        to: workers_address,
        value: 0u32.into(),
        data: input,
        signature: None, // Not signed. Yet.
    };

    // TODO: use network_id?
    // let network_id = chain.network_id;
    let tx = tx.sign(&private_key, None).to_bytes();
    let tx = hex::encode(tx);

    Ok(format!("0x{}", tx))
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk_test::marine_test;

    use crate::chain::chain_info::ChainInfo;
    use crate::hex::decode_hex;
    use crate::jsonrpc::register_worker::{encode_call, function, get_gas_price, make_tx};

    fn pat_id() -> Vec<u8> {
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

        let input = encode_call(pat_id(), WORKER_ID).expect("encode call");
        let input = hex::encode(input);

        assert_eq!(input, "d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5");
    }

    #[test]
    fn gen_tx() {
        let call = encode_call(pat_id(), WORKER_ID).expect("encode call");
        let chain = ChainInfo {
            workers: WORKERS.to_string(),
            workers_gas: 210000,
            wallet_key: PRIVATE_KEY.to_string(),
            ..ChainInfo::default()
        };
        let gas_price = get_gas_price().expect("get gas price");
        let tx = make_tx(call, chain, 3, gas_price).expect("make_tx");

        println!("tx_bytes 0x{}", tx);
    }

    // Set env RUST_LOGGER="mockito=debug" to enable Mockito's logs
    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn register(connector: marine_test_env::fluence_aurora_connector::ModuleInterface) {
        let _ = ::env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .filter_module("mockito", log::LevelFilter::Debug)
            .filter_module("fluence_aurora_connector", log::LevelFilter::Debug)
            .filter_module("marine_core", log::LevelFilter::Debug)
            .filter_module("wasmer_interface_types_fl", log::LevelFilter::Off)
            .is_test(true)
            .try_init();

        let jsonrpc = r#"
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": "0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05"
        }
        "#;

        // Create a mock
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/")
            .with_body_from_request(move |req| {
                println!("req: {:?}", req);
                jsonrpc.into()
            })
            // expect to receive this exact body in POST
            // .match_body(r#"{"jsonrpc":"2.0","id":0,"method":"eth_getLogs","params":[{"fromBlock":"0x52","toBlock":"0x246","address":"0x6328bb918a01603adc91eae689b848a9ecaef26d","topics":["0x55e61a24ecdae954582245e5e611fb06905d6af967334fff4db72793bebc72a9","0x7a82a5feefcaad4a89c689412031e5f87c02b29e3fced583be5f05c7077354b7"]}]}"#)
            // expect exactly 1 POST request
            .expect(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .create();

        let invalid_mock = server
            .mock("POST", "/")
            .expect(0)
            .with_status(404)
            .with_body("invalid mock was hit. Check that request body matches 'match_body' clause'")
            .create();

        let chain = marine_test_env::fluence_aurora_connector::ChainInfo {
            api_endpoint: url,
            deal_factory: "0x6328bb918a01603adc91eae689b848a9ecaef26d".into(),
            matcher: "0x6328bb918a01603adc91eae689b848a9ecaef26d".into(),
            workers: WORKERS.into(),
            workers_gas: 210_000,
            wallet_key: PRIVATE_KEY.into(),
        };
        let cp = CallParameters {
            init_peer_id: "".to_string(),
            service_id: "".to_string(),
            service_creator_peer_id: "".to_string(),
            host_id: "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE".to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };
        let result = connector.register_worker_cp(pat_id().into(), WORKER_ID.into(), chain, cp);
        println!("result: {:?}", result);

        // assert that there was no invalid requests
        invalid_mock.assert();

        // TODO: how to check request body?
        // check that mock was called
        mock.assert();
    }
}

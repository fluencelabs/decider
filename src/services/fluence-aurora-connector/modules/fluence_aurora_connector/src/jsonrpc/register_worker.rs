use std::convert::TryInto;

use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_tx_sign::{LegacyTransaction, Transaction};
use hex::FromHexError;
use marine_rs_sdk::marine;
use thiserror::Error;

use crate::chain::chain_info::ChainInfo;
use crate::hex::decode_hex;
use crate::jsonrpc::register_worker::RegisterWorkerError::{
    DecodeWorkersAddr, EncodeArgument, InvalidPrivateKey, InvalidWorkersAddr, SignTransaction,
};

#[derive(Debug, Error)]
pub enum RegisterWorkerError {
    #[error("error encoding function argument {1}: {0:?}")]
    EncodeArgument(FromHexError, &'static str),
    #[error("error encoding function inputs: {0:?}")]
    EncodeInput(#[from] ethabi::Error),
    #[error("error decoding WorkersModule contract address: {0:?}")]
    DecodeWorkersAddr(#[from] FromHexError),
    #[error("invalid workers addr: '{0}'. Must be of length 20, was {1}")]
    InvalidWorkersAddr(String, usize),
    #[error("error parsing private key from hex")]
    InvalidPrivateKey,
    #[error("Error signing tx: {0:?}")]
    SignTransaction(ethereum_tx_sign::Error),
}

#[marine]
pub fn register_worker(pat_id: &str, worker_id: &str, chain: ChainInfo) -> Vec<String> {
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
        Err(err) => vec![format!("{:?}", err)],
    }
}

/// Send transaction to RPC
fn send_tx(_tx: String, _api_endpoint: String) -> Result<(), RegisterWorkerError> {
    Ok(())
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
fn encode_call(pat_id: &str, worker_id: &str) -> Result<Vec<u8>, RegisterWorkerError> {
    let pat_id = hex::decode(pat_id).map_err(|e| EncodeArgument(e, "pat_id"))?;
    let pat_id = Token::FixedBytes(pat_id);

    let worker_id = hex::decode(worker_id).map_err(|e| EncodeArgument(e, "worker_id"))?;
    let worker_id = Token::FixedBytes(worker_id);

    let input = function().encode_input(&[pat_id, worker_id])?;
    Ok(input)
}

/// Load gas price from RPC
fn get_gas_price() -> Result<u128, RegisterWorkerError> {
    Ok(1_000_000)
}

/// Construct and sign transaction
fn make_tx(
    input: Vec<u8>,
    chain: ChainInfo,
    nonce: u128,
    gas_price: u128,
) -> Result<String, RegisterWorkerError> {
    let address = decode_hex(&chain.workers).map_err(DecodeWorkersAddr)?;
    let len = address.len();
    let address = address
        .try_into()
        .map_err(|_| InvalidWorkersAddr(chain.workers.clone(), len))?;

    let private_key = decode_hex(&chain.wallet_key).map_err(|_| InvalidPrivateKey)?;
    debug_assert_eq!(private_key.len(), 32);

    let tx = LegacyTransaction {
        chain: chain.network_id,
        nonce,
        to: Some(address),
        value: 0,
        gas_price,
        gas: chain.workers_gas,
        data: input,
    };

    let ecdsa = tx.ecdsa(&private_key).map_err(SignTransaction)?;
    let tx_bytes = tx.sign(&ecdsa);

    Ok(hex::encode(tx_bytes))
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use ethereum_tx_sign::Transaction;

    use crate::jsonrpc::register_worker::{encode_call, function};

    #[test]
    fn gen_call() {
        // function setWorker(bytes32 patId, bytes32 workerId) external
        let f = function();
        assert_eq!(f.signature(), "setWorker(bytes32,bytes32)");
        let signature_hex = hex::encode(f.short_signature());
        assert_eq!(signature_hex, "d5053ab0");

        let pat_id = "e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e";
        let worker_id = "529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5";
        let input = encode_call(pat_id, worker_id).expect("encode call");
        let input = hex::encode(input);
        assert_eq!(input, "d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5");
    }

    #[test]
    fn gen_tx() {
        use ethereum_tx_sign::LegacyTransaction;

        let pat_id = "e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e";
        let worker_id = "529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5";
        let input = encode_call(pat_id, worker_id).expect("encode call");

        let address = hex::decode("908aEBfb6051Bca6d1e684586d7760e53C4c736C")
            .expect("decode Matcher addr from hex");
        let address = address.try_into().expect("convert address to fixed array");

        let tx = LegacyTransaction {
            chain: 31337,
            nonce: 1,
            to: Some(address),
            value: 0,
            gas_price: 50000,
            gas: 210000,
            data: input,
        };

        let private_key = "bb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a";
        let private_key = hex::decode(private_key).expect("decode private key from hex");
        assert_eq!(private_key.len(), 32);
        let ecdsa = tx.ecdsa(&private_key).expect("calculate signature");
        let tx_bytes = tx.sign(&ecdsa);
        println!("tx_bytes 0x{}", hex::encode(tx_bytes));
    }
}
use marine_rs_sdk::marine;

use crate::jsonrpc::register_worker::RegisterWorkerError::EncodeArgument;
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use hex::FromHexError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterWorkerError {
    #[error("error encoding function argument {1}: {0:?}")]
    EncodeArgument(FromHexError, &'static str),
    #[error("error encoding function inputs: {0:?}")]
    EncodeInput(#[from] ethabi::Error),
}

#[marine]
pub fn register_worker() {
    unimplemented!()
}

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

fn encode_call(pat_id: &str, worker_id: &str) -> Result<Vec<u8>, RegisterWorkerError> {
    let pat_id = hex::decode(pat_id).map_err(|e| EncodeArgument(e, "pat_id"))?;
    let pat_id = Token::FixedBytes(pat_id);

    let worker_id = hex::decode(worker_id).map_err(|e| EncodeArgument(e, "worker_id"))?;
    let worker_id = Token::FixedBytes(worker_id);

    let input = function().encode_input(&[pat_id, worker_id])?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use crate::jsonrpc::register_worker::{encode_call, function};
    use ethereum_tx_sign::Transaction;
    use std::convert::TryInto;

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

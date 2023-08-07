use marine_rs_sdk::marine;

#[marine]
pub fn register_worker() {}

#[cfg(test)]
mod tests {
    use ethabi::{Function, Param, ParamType, StateMutability, Token};

    #[test]
    fn gen_tx() {
        // function setWorker(bytes32 patId, bytes32 workerId) external
        #[allow(deprecated)]
        let f = Function {
            name: "setWorker".into(),
            inputs: vec![
                Param {
                    name: "patId".to_string(),
                    kind: ParamType::FixedBytes(32),
                    internal_type: None,
                },
                Param {
                    name: "workerId".to_string(),
                    kind: ParamType::FixedBytes(32),
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::NonPayable,
        };

        assert_eq!(f.signature(), "setWorker(bytes32,bytes32)");
        let signature_hex = hex::encode(f.short_signature());
        assert_eq!(signature_hex, "d5053ab0");

        let pat_id = "e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e";
        let pat_id = hex::decode(pat_id).expect("decode pat_id from hex");
        let worker_id = "529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5";
        let worker_id = hex::decode(worker_id).expect("decode worker_id from hex");
        let input = f
            .encode_input(&[Token::FixedBytes(pat_id), Token::FixedBytes(worker_id)])
            .expect("encode input");
        let input = hex::encode(input);
        assert_eq!(input, "d5053ab0e532c726aa9c2f223fb21b5a488f874583e809257685ac3c40c9e0f7c89c082e529d4dabfa72abfd83c48adca7a2d49a921fa7351689d12e2a6c68375052f0b5");
    }
}

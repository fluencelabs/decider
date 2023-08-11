use marine_rs_sdk::marine;

use ethabi::{Function, Param, ParamType, StateMutability};

#[marine]
#[derive(Clone, Debug)]
pub struct Worker {
    host_id: String,
    worker_id: String,
}

#[marine]
#[derive(Clone, Debug)]
pub struct Subnet {
    workers: Vec<Worker>,
}

/// Description of the `getPATs` function from the `chain.workers` smart contract on chain
fn function() -> Function {
    #[allow(deprecated)]
    Function {
        name: String::from("getPATs"),
        inputs: vec![],
        outputs: vec![],
        constant: None,
        state_mutability: StateMutability::View,
    }
}

pub fn resolve_subnet(deal_id: String) {
    let input = function().encode_input(&[]).expect("encode input");
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_call() {}
}

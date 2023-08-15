use ethabi::ParamType::{Address, Array, FixedBytes, Tuple, Uint};
use ethabi::{Function, ParamType, StateMutability, Token};
use libp2p_identity::ParseError;
use marine_rs_sdk::marine;
use serde_json::json;
use thiserror::Error;

use crate::chain::chain_data::ChainDataError::InvalidParsedToken;
use crate::chain::chain_data::{parse_chain_data, ChainDataError};
use crate::chain::data_tokens::next_opt;
use crate::curl::send_jsonrpc;
use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::resolve_subnet::ResolveSubnetError::{Empty, InvalidPeerId};
use crate::jsonrpc::{JsonRpcError, JsonRpcReq, JSON_RPC_VERSION};
use crate::peer_id::parse_peer_id;

#[derive(Error, Debug)]
pub enum ResolveSubnetError {
    #[error("error encoding function: '{0}'")]
    EncodeFunction(#[from] ethabi::Error),
    #[error("error sending jsonrpc request: '{0}'")]
    SendRPC(#[from] RequestError),
    #[error("error sending jsonrpc request: '{0}'")]
    ReceiveRPC(#[from] JsonRpcError),
    #[error(transparent)]
    ChainData(#[from] ChainDataError),
    #[error("'{0}' not found in getPATs response")]
    MissingField(&'static str),
    #[error("getPATs response is empty")]
    Empty,
    #[error("'{1}' from getPATs is not a valid PeerId")]
    InvalidPeerId(#[source] ParseError, &'static str),
}

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
    error: Vec<String>,
}

fn signature() -> ParamType {
    Array(Box::new(Tuple(vec![
        // bytes32 id
        FixedBytes(32),
        // uint256 index
        Uint(256),
        // bytes32 peerId
        FixedBytes(32),
        // bytes32 workerId
        FixedBytes(32),
        // address owner
        Address,
        // uint256 collateral
        Uint(256),
        // uint256 created
        Uint(256),
    ])))
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

fn decode_pats(data: String) -> Result<Vec<Worker>, ResolveSubnetError> {
    let tokens = parse_chain_data(&data, &[signature()])?;
    let tokens = tokens.into_iter().next().ok_or(Empty)?;
    let tokens = tokens.into_array().ok_or(InvalidParsedToken("response"))?;
    let mut result = vec![];
    for token in tokens {
        let tuple = token.into_tuple().ok_or(InvalidParsedToken("tuple"))?;
        let mut tuple = tuple.into_iter().skip(2);

        let peer_id = next_opt(&mut tuple, "compute_peer_id", Token::into_fixed_bytes)?;
        let peer_id = parse_peer_id(peer_id).map_err(|e| InvalidPeerId(e, "compute_peer_id"))?;
        let worker_id = next_opt(&mut tuple, "compute_worker_id", Token::into_fixed_bytes)?;
        let worker_id = parse_peer_id(worker_id).map_err(|e| InvalidPeerId(e, "worker_id"))?;

        let pat = Worker {
            host_id: peer_id.to_string(),
            worker_id: worker_id.to_string(),
        };
        result.push(pat);
    }

    Ok(result)
}

#[marine]
pub fn resolve_subnet(deal_id: String, api_endpoint: &str) -> Subnet {
    let res: Result<_, ResolveSubnetError> = try {
        let input = function().encode_input(&[])?;
        let input = format!("0x{}", hex::encode(input));
        let req = JsonRpcReq {
            jsonrpc: JSON_RPC_VERSION.into(),
            id: 0,
            method: "eth_call".to_string(),
            params: vec![
                json!({ "data": input, "to": deal_id }).to_string(),
                "latest".to_string(),
            ],
        };
        let response = send_jsonrpc(api_endpoint, req)?;
        let pats = response.get_result()?;

        decode_pats(pats)?
    };

    match res {
        Ok(workers) => Subnet {
            workers,
            error: vec![],
        },
        Err(err) => Subnet {
            workers: vec![],
            error: vec![format!("{}", err)],
        },
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    // Set env RUST_LOGGER="mockito=debug" to enable Mockito's logs
    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn get_pats(connector: marine_test_env::fluence_aurora_connector::ModuleInterface) {
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
            "result": "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000032b7083358039745e731fb9809204d9304b48797406593e180b4e5a762a47321400000000000000000000000000000000000000000000000000000000000000012623d2cc0692ce6cb68ab094f95daa06a92a36f3cf7190e9baf7dd61562899f4a510574bbf0159ca28b7fb191d252346d1a32f853a3f0b1c9c5e59e28cfd546c0000000000000000000000000b9b9ac40dc527ea6a98110b796b0007074d49dd0000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000004fdbfb375f013a592c50174ad241c67a4cf1b9ec81c902900b75f801f83cd2657a00000000000000000000000000000000000000000000000000000000000000022623d2cc0692ce6cb68ab094f95daa06a92a36f3cf7190e9baf7dd61562899f400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b9b9ac40dc527ea6a98110b796b0007074d49dd0000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000004fec7c6fea91d971bc7c5ed340ec86265bb93386fff248e842a1a69a94b58d2d9e00000000000000000000000000000000000000000000000000000000000000032623d2cc0692ce6cb68ab094f95daa06a92a36f3cf7190e9baf7dd61562899f400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b9b9ac40dc527ea6a98110b796b0007074d49dd0000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000004f"
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

        let subnet =
            connector.resolve_subnet("0x6dD1aFfe90415C61AeDf5c0ACcA9Cf5fD5031517".into(), url);

        let pats: Vec<_> = subnet
            .workers
            .iter()
            .map(|p| (p.host_id.as_str(), p.worker_id.as_str()))
            .collect();

        assert_eq!(
            pats,
            vec![
                (
                    "12D3KooWCPFLtcLwzT1k4gsacu3gkM2gYJTXdnTSfsPFZ67FrD4F",
                    "12D3KooWLvhtdbBuFTzxvDXUGYcyxyeZrab1tZWEY4YY8K6PTjTH"
                ),
                (
                    "12D3KooWCPFLtcLwzT1k4gsacu3gkM2gYJTXdnTSfsPFZ67FrD4F",
                    "12D3KooW9pNAk8aiBuGVQtWRdbkLmo5qVL3e2h5UxbN2Nz9ttwiw"
                ),
                (
                    "12D3KooWCPFLtcLwzT1k4gsacu3gkM2gYJTXdnTSfsPFZ67FrD4F",
                    "12D3KooW9pNAk8aiBuGVQtWRdbkLmo5qVL3e2h5UxbN2Nz9ttwiw"
                ),
            ]
        );

        // assert that there was no invalid requests
        invalid_mock.assert();

        // TODO: how to check request body?
        // check that mock was called
        mock.assert();
    }
}

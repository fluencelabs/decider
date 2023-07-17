use marine_rs_sdk::marine;

use crate::chain::chain_data::{unhex, ChainData};
use crate::chain::chain_info::ChainInfo;
use crate::chain::deal_matched::{DealMatched, Match};
use crate::chain::log::parse_logs;
use crate::jsonrpc::get_logs::get_logs;
use crate::jsonrpc::request::check_url;
use crate::jsonrpc::right_boundary::default_right_boundary;

#[marine]
#[derive(Clone, Debug)]
pub struct MatchedResult {
    error: Vec<String>,
    success: bool,
    logs: Vec<DealMatched>,
    /// The response contains logs for blocks from `left_boundary` to `right_boundary`
    right_boundary: String,
}

impl MatchedResult {
    pub fn ok(logs: Vec<DealMatched>, right_boundary: String) -> Self {
        Self {
            success: true,
            error: vec![],
            logs,
            right_boundary,
        }
    }

    pub fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            logs: vec![],
            right_boundary: String::new(),
        }
    }
}

// TODO: How to set an upper limit for how many responses to return?
//       Don't see this functionallity in eth_getLogs
// TODO: need to restrict who can use this service to its spell
//
// `api_endpoint` -- api endpoint to poll (right now it's possible to pass any URL for emergency cases)
// `address`      -- address of the chain contract
// `left_boundary`   -- from which block to poll deals
#[marine]
pub fn poll_deal_matches(chain: ChainInfo, left_boundary: String) -> MatchedResult {
    if let Err(err) = check_url(&chain.api_endpoint) {
        return MatchedResult::error(err.to_string());
    }

    let right_boundary = default_right_boundary(&left_boundary);
    // pad provider to 32 bytes
    let provider = format!("0x{:0>64}", unhex(chain.provider));
    let logs = get_logs(
        chain.api_endpoint,
        chain.matcher,
        left_boundary,
        right_boundary.clone(),
        vec![Match::topic(), provider],
    );

    match logs {
        Err(err) => return MatchedResult::error(err.to_string()),
        Ok(logs) => {
            let matches = parse_logs::<Match, DealMatched>(logs);
            MatchedResult::ok(matches, right_boundary)
        }
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    // modules_dir = "../../../../../../../target/wasm32-wasi/release/"
    fn poll(connector: marine_test_env::fluence_aurora_connector::ModuleInterface) {
        let jsonrpc = r#"
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": [
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x1a7122fa7501f09f19f29451548e88adf7ec88c99d34b4abdd09b27dfdbd74f1",
                    "blockHash": "0x1c6808f9f4f99bdad9a63601e07230b84effaec5aba724963ef17651131cf75d",
                    "blockNumber": "0x4e",
                    "address": "0x6328bb918a01603adc91eae689b848a9ecaef26d",
                    "data": "0x00000000000000000000000099e28f59ddfe14ff4e598a3ba3928bbf87b3f2b30000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000004d0155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e3",
                    "topics": [
                        "0x8a2ecab128faa476aff507c7f34da3348b5c56e4a0401825f6919b4cc7b249f1",
                        "0x0000000000000000000000006f10e8209296ea9e556f80b0ff545d8175f271d0"
                    ]
                }
            ]
        }
        "#;

        // Create a mock
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/")
            // expect to receive this exact body in POST
            .match_body(r#"{"jsonrpc":"2.0","id":0,"method":"eth_getLogs","params":[{"fromBlock":"0x52","toBlock":"0x246","address":"0x6328bb918a01603adc91eae689b848a9ecaef26d","topics":["0x8a2ecab128faa476aff507c7f34da3348b5c56e4a0401825f6919b4cc7b249f1","0x0000000000000000000000006f10e8209296ea9e556f80b0ff545d8175f271d0"]}]}"#)
            // expect exactly 1 POST request
            .expect(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(jsonrpc)
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
            provider: "0x6f10e8209296ea9e556f80b0ff545d8175f271d0".to_string(),
        };
        let result = connector.poll_deal_matches(chain, "0x52".into());

        assert!(result.success, "poll failed: {:?}", result);
        assert_eq!(
            result.logs.len(),
            1,
            "expected 1 logs, got {}",
            result.logs.len()
        );
        let log = result.logs.into_iter().next().unwrap().info;
        assert_eq!(
            log.compute_provider.to_lowercase(),
            "0x6f10e8209296ea9e556f80b0ff545d8175f271d0".to_lowercase()
        );
        assert_eq!(
            log.deal_id.to_lowercase(),
            "0x99e28F59DdfE14fF4e598a3Ba3928bbF87b3f2B3".to_lowercase()
        );

        // assert that there was no invalid requests
        invalid_mock.assert();

        // TODO: how to check request body?
        // check that mock was called
        mock.assert();
    }
}

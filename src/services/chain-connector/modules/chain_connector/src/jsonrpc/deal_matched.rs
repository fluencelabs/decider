use libp2p_identity::PeerId;
use marine_rs_sdk::marine;
use std::str::FromStr;

use crate::chain::chain_data::ChainData;
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
    use marine_rs_sdk::get_call_parameters;

    let host = get_call_parameters().host_id;
    let host = PeerId::from_str(&host).expect("parse host_id to peer_id");
    let host: Vec<_> = host.to_bytes().into_iter().skip(6).collect();
    let host = format!("0x{:0>64}", hex::encode(host));

    if let Err(err) = check_url(&chain.api_endpoint) {
        return MatchedResult::error(err.to_string());
    }

    let right_boundary = default_right_boundary(&left_boundary);
    let logs = get_logs(
        &chain.api_endpoint,
        chain.matcher,
        left_boundary,
        right_boundary.clone(),
        vec![Match::topic(), host],
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
    use libp2p_identity::PeerId;
    use marine_rs_sdk_test::CallParameters;
    use marine_rs_sdk_test::marine_test;
    use std::str::FromStr;

    #[test]
    fn serialize_peer_id() {
        let host = "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE";
        let host = PeerId::from_str(&host).expect("parse host_id to peer_id");
        println!("host: {}", host);
        let host: Vec<_> = host.to_bytes().into_iter().skip(6).collect();
        println!("host: {:?}", host);
        let host = format!("0x{:0>64}", hex::encode(host));
        println!("host: {}", host);
    }

    // Set env RUST_LOGGER="mockito=debug" to enable Mockito's logs
    #[marine_test(config_path = "../../../../../../../src/distro/decider-spell/Config.toml")]
    fn poll(connector: marine_test_env::chain_connector::ModuleInterface) {
        let _ = ::env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .filter_module("mockito", log::LevelFilter::Debug)
            .filter_module("chain_connector", log::LevelFilter::Debug)
            .filter_module("marine_core", log::LevelFilter::Debug)
            .filter_module("wasmer_interface_types_fl", log::LevelFilter::Off)
            .is_test(true)
            .try_init();

        let jsonrpc = r#"
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": [
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0xe3943cc5057c8ed33ec9a6891421b367d0f8179b167559ca6e1dae9992941003",
                    "blockHash": "0xa26b32fbefcf53e5484c0325fd6da72ee03c7198f1c32b4f8b4582b93525837b",
                    "blockNumber": "0x51",
                    "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                    "data": "0x000000000000000000000000ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b9900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3",
                    "topics": [
                        "0x55e61a24ecdae954582245e5e611fb06905d6af967334fff4db72793bebc72a9",
                        "0x7a82a5feefcaad4a89c689412031e5f87c02b29e3fced583be5f05c7077354b7"
                    ]
                },
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x093f57ec0df3420f3c8a52ee90fa9ef05aed9827fa05ba6e997bdd4b1b982189",
                    "blockHash": "0xd2f21035758026e7f0be21c13278b1d4f993b6d75647b0c29d431a4f271ccfd0",
                    "blockNumber": "0x57",
                    "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                    "data": "0x00000000000000000000000067b2ad3866429282e16e55b715d12a77f85b7ce800000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000560155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3",
                    "topics": [
                        "0x55e61a24ecdae954582245e5e611fb06905d6af967334fff4db72793bebc72a9",
                        "0x7a82a5feefcaad4a89c689412031e5f87c02b29e3fced583be5f05c7077354b7"
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
            .match_body(r#"{"jsonrpc":"2.0","id":0,"method":"eth_getLogs","params":[{"fromBlock":"0x52","toBlock":"0x246","address":"0x6328bb918a01603adc91eae689b848a9ecaef26d","topics":["0x55e61a24ecdae954582245e5e611fb06905d6af967334fff4db72793bebc72a9","0x7a82a5feefcaad4a89c689412031e5f87c02b29e3fced583be5f05c7077354b7"]}]}"#)
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

        let compute_peer = "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE";
        let chain = marine_test_env::chain_connector::ChainInfo {
            api_endpoint: url,
            matcher: "0x6328bb918a01603adc91eae689b848a9ecaef26d".into(),
            workers_gas: <_>::default(),
            wallet_key: <_>::default(),
            network_id: 80001,
        };
        let cp = CallParameters {
            init_peer_id: "".to_string(),
            service_id: "".to_string(),
            service_creator_peer_id: "".to_string(),
            host_id: compute_peer.to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };
        let result = connector.poll_deal_matches_cp(chain, "0x52".into(), cp);

        assert!(result.success, "poll failed: {:?}", result);
        assert_eq!(
            result.logs.len(),
            2,
            "expected 1 logs, got {}",
            result.logs.len()
        );
        let log = result.logs.into_iter().next().unwrap().info;
        assert_eq!(log.compute_peer.to_lowercase(), compute_peer.to_lowercase());
        assert_eq!(
            log.deal_id.to_lowercase(),
            "0xffa0611a099ab68ad7c3c67b4ca5bbbee7a58b99".to_lowercase()
        );

        // assert that there was no invalid requests
        invalid_mock.assert();

        // TODO: how to check request body?
        // check that mock was called
        mock.assert();
    }
}

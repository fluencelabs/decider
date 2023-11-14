#![feature(async_closure)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

pub mod utils;

use crate::utils::test_rpc_server::run_test_server;
use crate::utils::*;
use fluence_spell_dtos::value::StringValue;
use maplit::hashmap;
use serde_json::json;
use utils::TestApp;

#[tokio::test]
async fn test_update_deal() {
    const BLOCK_INIT: u32 = 10;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();
    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    {
        let expected_reqs = 6;
        for _ in 0..expected_reqs {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => json!(to_hex(BLOCK_INIT)),
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    json!([TestApp::log_test_app2(
                        DEAL_ID,
                        BLOCK_NUMBER,
                        log.topics[1].as_str()
                    )])
                }
                "eth_sendRawTransaction" => {
                    json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
                }
                "eth_getTransactionCount" => json!("0x1"),
                "eth_gasPrice" => json!("0x3b9aca07"),
                "eth_getTransactionReceipt" => default_receipt(),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
        }
    }
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join only one deal");
    let deal = deals.remove(0);

    // run again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            server.send_response(Ok(json!("0x200")));
        }
        // no new deals
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
    }
    // deal update phase
    {
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_getLogs");
        let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
        assert_eq!(
            log.address, deal.deal_id,
            "wrong deal_id in the update-deal request"
        );

        let response = json!( [
              {
                "address": deal.deal_id,
                "topics": [
                  "0x0e85c04920a2349be7d0f03a765fa172e5dabc0a4a9fc47acb81c07ce8d260d0",
                ],
                  // CID of the app from test_app_1
                "data": "0x0155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e3",

                "blockNumber": "0x300",
                "transactionHash": "0xb825edf7da59840ce838a9ed70aa0aa6c54c322ca5d6f0be4f070766e46ebbd8",
                "transactionIndex": "0xb",
                "blockHash": "0x34ba65babca6f1ef44da5f75c7bb4335c7b7484178a74003de5df139ac6551ed",
                "logIndex": "0x26",
                "removed": false
              }
            ]
        );
        server.send_response(Ok(json!(response)));
    }
    wait_decider_stopped(&mut client).await;

    let cid = {
        let result = execute(
            &mut client,
            r#"(seq
                (call relay ("op" "noop") [])
                (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
            )"#,
            "cid",
            hashmap! { "worker_id" => json!(deal.worker_id )},
        )
        .await
        .unwrap();
        let result = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
        assert!(!result.absent, "no `worker_def_cid` on worker-spell");
        serde_json::from_str::<String>(&result.str).unwrap()
    };
    let original_app = TestApp::test_app2();
    let new_app = TestApp::test_app1();
    assert_ne!(cid, original_app.cid, "CID must be changed");
    assert_eq!(cid, new_app.cid, "CID must be set to the new app");

    server.shutdown().await
}

#[tokio::test]
async fn test_update_deal_from_later_blocks() {
    enable_decider_logs();
    const LATEST_BLOCK_INIT: u32 = 50;
    // should be less than LATEST
    const BLOCK_NUMBER_DEAL: u32 = 40;
    // for the window to move, must be greater then the window size
    const LATEST_BLOCK_FIRST_UPDATE: u32 = LATEST_BLOCK_INIT + DEFAULT_POLL_WINDOW_BLOCK_SIZE;
    // some block number in the expected window
    const LATEST_BLOCK_SECOND_UPDATE: u32 =
        BLOCK_NUMBER_DEAL + DEFAULT_POLL_WINDOW_BLOCK_SIZE + 100;
    const DEAL_ID: &'static str = DEAL_IDS[0];

    let mut server = run_test_server();
    let url = server.url.clone();
    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    {
        let expected_reqs = 6;
        for _ in 0..expected_reqs {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => json!(to_hex(LATEST_BLOCK_INIT)),
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    json!([TestApp::log_test_app2(
                        DEAL_ID,
                        BLOCK_NUMBER_DEAL,
                        log.topics[1].as_str()
                    )])
                }
                "eth_sendRawTransaction" => {
                    json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
                }
                "eth_getTransactionCount" => json!("0x1"),
                "eth_gasPrice" => json!("0x3b9aca07"),
                "eth_getTransactionReceipt" => default_receipt(),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
        }
    }
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join only one deal");

    // Check that the queried window is moving
    update_config(&mut client, &oneshot_config()).await.unwrap();
    let right_boundary_prev_update = {
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            server.send_response(Ok(json!(to_hex(LATEST_BLOCK_FIRST_UPDATE))));
        }
        // no new deals
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
        // updates for deals
        {
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(
                BLOCK_NUMBER_DEAL, log.from_block,
                "deal update should be polled from the block where the deal was deployed"
            );

            server.send_response(Ok(json!([])));
            log.to_block
        }
    };
    wait_decider_stopped(&mut client).await;

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            server.send_response(Ok(json!(to_hex(LATEST_BLOCK_SECOND_UPDATE))));
        }
        // no new deals
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
        // updates for deals
        {
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(
                right_boundary_prev_update + 1, log.from_block,
                "poll should be started from the to_block + 1, from the last iteration, since the window should move (window size {} blocks)",
                DEFAULT_POLL_WINDOW_BLOCK_SIZE
            );

            server.send_response(Ok(json!([])));
        }
    }
    wait_decider_stopped(&mut client).await;

    server.shutdown().await
}

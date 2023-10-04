#![feature(async_closure)]
#![feature(try_blocks)]

pub mod utils;

use utils::test_rpc_server::run_test_server;

use crate::utils::{
    execute, get_joined_deals, make_distro_default, make_distro_with_api,
    make_distro_with_api_and_config, oneshot_config, package_items_names, to_hex, update_config,
    update_decider_script_for_tests, wait_decider_stopped, LogsReq, TestApp, DEAL_IDS,
};
use connected_client::ConnectedClient;
use created_swarm::make_swarms_with_cfg;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::{StringValue, UnitValue};
use maplit::hashmap;
use serde::Serialize;
use serde_json::{json, Value};

#[test]
fn test_connector_config_check() {
    let connector = decider_distro::connector_service_modules();
    let marine_config: Result<TomlMarineConfig, _> = toml::from_slice(connector.config);
    assert!(
        marine_config.is_ok(),
        "connector marine config is not valid toml"
    );
}

#[tokio::test]
async fn test_decider_installed() {
    let distro = make_distro_default();
    let names = package_items_names(&distro);
    assert_eq!(
        names.len(),
        2,
        "expect only 2 services and spells in the decider package"
    );

    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        // disable built-in system services (disabled by default for now, but just in case)
        cfg.enabled_system_services = vec![];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;
    let mut client = ConnectedClient::connect_to(swarms[0].multiaddr.clone())
        .await
        .unwrap();

    let result = client
        .execute_particle(
            r#"
        (seq
          (call relay ("srv" "list") [] list)
          (call client ("return" "") [list])
        )
       "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string())
            },
        )
        .await
        .unwrap();

    if let [Value::Object(service1), Value::Object(service2)] =
        result[0].as_array().expect("expect an array").as_slice()
    {
        let alias1 = service1["aliases"].as_array().unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        let alias2 = service2["aliases"].as_array().unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        assert!(names.contains(&alias1), "{alias1} service is not installed");
        assert!(names.contains(&alias2), "{alias2} service is not installed");
    }
}

// Update doesn't work and we don't know how it will work in future
// Maybe, make this test ALWAYS fail to remind that this doesn't work?
#[tokio::test]
async fn test_update_deal() {
    const BLOCK_INIT: u32 = 10;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let (server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;

    let mut client = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();

    update_decider_script_for_tests(&mut client, swarms[0].tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    {
        let expected_reqs = 5;
        for _ in 0..expected_reqs {
            let (method, params) = recv_request.recv().await.unwrap();
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
                _ => panic!("unexpected method: {}", method),
            };
            send_response.send(Ok(response)).unwrap();
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
            let (method, _) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            send_response.send(Ok(json!("0x200"))).unwrap();
        }
        // no new deals
        {
            let (method, _) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            send_response.send(Ok(json!([]))).unwrap();
        }
    }
    // deal update phase
    {
        let (method, params) = recv_request.recv().await.unwrap();
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
        send_response.send(Ok(response)).unwrap();
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
async fn test_remove_deal() {
    const BLOCK_INIT: u32 = 10;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let (server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;

    let mut client = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();

    update_decider_script_for_tests(&mut client, swarms[0].tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    {
        let expected_reqs = 5;
        for _ in 0..expected_reqs {
            let (method, params) = recv_request.recv().await.unwrap();
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
                _ => panic!("unexpected method: {}", method),
            };
            send_response.send(Ok(response)).unwrap();
        }
    }
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join only one deal");
    let deal = deals.remove(0);

    // put remove_deal message to Decider's mailbox
    #[derive(Serialize)]
    struct Worker {
        host_id: String,
        worker_id: String,
    }
    #[derive(Serialize)]
    struct RemoveMsg {
        remove: Vec<Worker>,
    }
    let host_id = client.node.to_string();
    let result = execute(
        &mut client,
        r#"
            (call relay ("decider" "push_mailbox") [remove_msg] result)
        "#,
        "result",
        hashmap! {
            "remove_msg" => json!(json!(RemoveMsg {
                remove: vec![Worker {
                    host_id: host_id,
                    worker_id: deal.worker_id.clone(),
                }]
            }).to_string())
        },
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<UnitValue>(result[0].clone()).unwrap();
    assert!(
        result.success,
        "couldn't push remove_deal message to Decider: {}",
        result.error
    );

    // run again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    for _step in 0..3 {
        let (method, _params) = recv_request.recv().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => json!("0x10"),
            "eth_getLogs" => {
                json!([])
            }
            _ => panic!("unexpected method: {}", method),
        };
        send_response.send(Ok(response)).unwrap();
    }
    wait_decider_stopped(&mut client).await;
    /*
    // Deals aren't removed from `joined_deals` atm
    let deals = get_joined_deals(&mut client).await;
    assert!(
        !deals.is_empty(),
        "deal removal must only remove entry from joined_deals"
    );
     */

    let workers = execute(
        &mut client,
        r#"
        (call relay ("worker" "list") [] workers)
        "#,
        "workers",
        hashmap! {},
    )
    .await
    .unwrap();
    let workers = serde_json::from_value::<Vec<String>>(workers[0].clone()).unwrap();
    assert!(!workers.contains(&deal.worker_id), "worker must be removed");

    server.shutdown().await
}

///
/// Test how *Decider* calculates the block to poll.
/// Block numbers sequence:
/// 1. `0x0`  -- Decider should be able to poll from the beginning and don't break,so
///            saved last_seen_block should be `0x0`
///            In the eth_getLogs request: we can check that fromBlock is `0x0`
///
/// 2. `0x10` -- the number is less then the range decider polls
///            Decider should move it's left boundary to this block, so
///            saved last_seen_block should be `0x10`
///            In the eth_getLogs request: we can check that fromBlock is `0x1`
///
/// 3. `0x10` -- again the same number in not very realistic case when Decider is too fast and the chain is too slow
///            Decider shouldn't move it's left boundary anywhere, so
///            saved last_seen_block should be `0x10`
///            In the eth_getLogs request: we can check that fromBlock is `0x11`
///
/// 4. `0xffffff` -- big number which Decider shouldn't be able to process during one iteration
///            Decider should move to some boundary configured boundary, which is 500 blocks, so
///            saved last_seen_block should be `0x205`
///            In the eth_getLogs request: we can check that fromBlock is `0x11`
///            Note: the test depend on the knowledge that the range is 500 blocks,
///                  we don't evaluate the number automatically
///
#[tokio::test]
async fn test_left_boundary_idle() {
    //enable_decider_logs();

    let (server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;
    let mut client = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();

    // To be able to wait 'til the end of one cycle
    update_decider_script_for_tests(&mut client, swarms[0].tmp_dir.clone()).await;

    let mut oneshot_config = TriggerConfig::default();
    oneshot_config.clock.start_sec = 1;

    let block_numbers = vec!["0x0", "0x10", "0x10", "0xffffff"];
    let expected_last_seen = vec!["0x0", "0x10", "0x10", "0x205"];
    let expected_from_blocks = vec!["0x0", "0x1", "0x11", "0x11"];

    for step in 0..block_numbers.len() {
        update_config(&mut client, &oneshot_config).await.unwrap();
        {
            let (method, params) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            assert!(params.is_empty());
            send_response.send(Ok(json!(block_numbers[step]))).unwrap();
        }

        {
            let (method, params) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            assert!(!params.is_empty());
            let log_req = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(to_hex(log_req.from_block), expected_from_blocks[step]);

            send_response.send(Ok(json!([]))).unwrap();
        }
        wait_decider_stopped(&mut client).await;

        let result = execute(
            &mut client,
            r#" (call relay ("decider" "get_string") ["last_seen_block"] last_seen) "#,
            "last_seen",
            hashmap! {},
        )
        .await
        .unwrap();
        let last_seen = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
        assert_eq!(last_seen.str, expected_last_seen[step]);
    }

    server.shutdown().await;
}

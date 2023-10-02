#![feature(async_closure)]
#![allow(warnings)]
#![feature(try_blocks)]

mod utils;

use utils::test_rpc_server::{run_test_server, run_test_server_predefined};
use utils::*;

use connected_client::ConnectedClient;
use created_swarm::make_swarms_with_cfg;
use eyre::WrapErr;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::{ScriptValue, StringListValue, StringValue, U32Value, UnitValue};
use hyper::body::Buf;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::future::Future;
use std::io::Read;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tracing::log::Log;

const DEAL_IDS: &[&'static str] = &[
    "ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b99",
    "880a53a54785df22ba804aee81ce8bd0d45bdedc",
    "67b2ad3866429282e16e55b715d12a77f85b7ce8",
    "1234563866429282e16e55b715d12a77f85b7cc9",
    "991b64a54785df22ba804aee81ce8bd0d45bdabb",
    "3665748409e712cd91b428c18e07a8e37b44c47e",
];

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
/// Test the basic flow
///
/// 1. *Decider* asks the last block of the chain from which to start polling
///    The block number is `0x10`
///
/// 2. *Decider* asks for the logs from the block `0x10` to the `0x10 + 500` blocks (range configured in the connector)
///    We return the logs with pre-defined CID of the url-downloader app. This step requires a working IPFS node with
///    the app uploaded.
///
/// 3. *Decider* creates a worker for the deal, deploys the worker-spell with the CID from the deal
///    and marks the deal joined
///    We check the `joined_deal` list in the KV that it contains:
///    - correct `deal_id` from the logs
///    - existing `worker_id` with a Worker Spell and the installed app from the deal
///    We check that the Worker Spell has the correct CID
///    TODO: check saved state for the deal
///
/// 4. *Decider* registers the worker on chain
///    TODO: We CAN check that the registration request is correct and contains the correct worker_id
///
/// 5. *Decider* updates the last_seen_block to the previous block from the processed one
///    to be sure that we don't miss any logs if we won't have time to process the whole list of deals,
///    so we can process them on the next iteration.
///
/// 6. *Decider* looks for the updates from the already joined deals (not the new ones) and checks mailbox.
///    In this test we don't have any updates or mailbox messages.
///
/// 7. After creation, *Worker Spell* downloads the app from IPFS and deploys it on the worker
///    Here, as a part of testing the basic flow, we check that the app was deployed correctly:
///    - `srv.list` on the worker returns the `worker-spell` spell and the `url-downloader` service
///
/// 8. *Decider* polls updates for already existing deals.
///    Check that *Decider* doesn't try to find updates for the new deal.
///
/// NOTE: This test REQUIRES an IPFS node to be up and have the url-downloader app uploaded.
/// TODO: provide the app in the tests resources
/// TODO: checks that `errors` are empty
///
#[tokio::test]
async fn test_deploy_a_deal_single() {
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK: u32 = 32;

    let mut server = run_test_server_predefined(async move |method, params| {
        match method.as_str() {
            "eth_blockNumber" => {
                json!("0x10")
            }
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                Value::Array(vec![TestApp::log_test_app1(
                    DEAL_ID,
                    BLOCK,
                    log.topics[1].as_str(),
                )])
            }
            "eth_sendRawTransaction" => {
                // how to check correctness of the subnet registration?
                json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
            }
            "eth_getTransactionCount" => json!("0x1"),
            "eth_gasPrice" => json!("0x3b9aca07"),
            _ => panic!("mock http got unexpected rpc method: {}", method),
        }
    });

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

    wait_decider_stopped(&mut client).await;

    let mut result = execute(
        &mut client,
        r#"
            (seq
                (call relay ("decider" "get_u32") ["counter"] counter)
                (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
            )
        "#,
        "counter deals",
        hashmap! {},
    )
    .await
    .unwrap();

    // Check that decider was really run
    let counter = serde_json::from_value::<U32Value>(result.remove(0)).unwrap();
    assert!(
        !counter.absent,
        "decider hasn't started yet (no counter in kv)"
    );
    assert_eq!(counter.num, 1, "decider wasn't run");

    // Analyse joined deals
    let deal = {
        let mut deals = parse_joined_deals(result.remove(0));
        assert_eq!(deals.len(), 1, "decider joined more than one deal");
        deals.remove(0)
    };

    // Check that we can find worker_id by deal_id
    let resolved_worker_id = {
        let mut result = execute(
            &mut client,
            r#"(call relay ("worker" "get_worker_id") [deal_id] resolved_worker_id)"#,
            "resolved_worker_id",
            hashmap! {
                "deal_id" => json!(deal.deal_id),
            },
        )
        .await
        .unwrap();
        let mut worker_id = serde_json::from_value::<Vec<String>>(result.remove(0)).unwrap();
        assert!(!worker_id.is_empty(), "can't resolve worker_id by deal_id");
        worker_id.remove(0)
    };

    assert_eq!(deal.worker_id, resolved_worker_id);
    assert_eq!(deal.deal_id, format!("0x{DEAL_ID}"));

    let state = get_deal_state(&mut client, &deal.deal_id).await;
    assert_eq!(
        state.left_boundary,
        to_hex(BLOCK),
        "wrong saved state for the deal"
    );

    // Check that we can see the list of services on the worker

    #[derive(Deserialize, Debug)]
    struct ServiceInfo {
        aliases: Vec<String>,
        worker_id: String,
        service_type: String,
    }

    // Here we also test that the Installation Spell worked correctly to ensure that the distro is fine,
    // but deep Installation Spell testing is out of scope of this test suits
    wait_worker_spell_stopped(
        &mut client,
        deal.worker_id.clone(),
        Duration::from_millis(200),
    )
    .await;

    let worker_service_list = {
        let result = execute(
            &mut client,
            r#"
            (seq
                (call relay ("op" "noop") [])
                (call worker_id ("srv" "list") [] list)
            )
        "#,
            "list",
            hashmap! {
                "worker_id" => json!(deal.worker_id),
            },
        )
        .await
        .unwrap();
        serde_json::from_value::<Vec<ServiceInfo>>(result[0].clone()).unwrap()
    };

    let test_app_1 = TestApp::test_app1();
    let worker_spell = worker_service_list
        .iter()
        .find(|info| info.aliases.contains(&"worker-spell".to_string()));
    assert!(worker_spell.is_some(), "no worker-spell on the worker");
    let worker_spell = worker_spell.unwrap();
    assert_eq!(
        worker_spell.service_type, "spell",
        "worker-spell is not a spell, it's {}",
        worker_spell.service_type,
    );
    assert_eq!(
        worker_spell.worker_id, deal.worker_id,
        "worker-spell has different worker_id"
    );

    // Check that the worker-spell has the same CID as we wanted to deploy
    // We do it since it's Decider's responsibility to set the correct CID
    let cid = get_worker_app_cid(&mut client, &deal.worker_id).await;
    assert_eq!(cid, test_app_1.cid, "Deal CID on worker-spell is different");

    // Then check that the app from CID was deployed
    let test_service = worker_service_list
        .iter()
        .find(|info| info.aliases.contains(&test_app_1.services_names[0]));
    assert!(
        test_service.is_some(),
        "no test service on the worker from a deal"
    );
    let test_service = test_service.unwrap();
    assert_eq!(
        test_service.service_type, "service",
        "test service is not a service, it's {}",
        test_service.service_type,
    );
    assert_eq!(
        test_service.worker_id, deal.worker_id,
        "test service has different worker_id"
    );

    server.shutdown().await
}

/// Test deal deployment when Decider finds several deals at the same time
///
///  1. *Decider* polls logs from the chain
///     We return the logs with several predefined deals
///     Check how *Decider* processes `last_seen_block`
///
///  2. *Decider* creates workers for the deals and provides the CIDs to the corresponsing worker-spells
///     Check that
///     a. *Decider* created workers for each deal
///     b. *Decider* set the correct CID to each `worker-spell`
///     c. *Decider* correctly saved info about the `joined_deals`
///  3. *Decider* registers the workers on chain
///     Check that
///     a. *Decider* try to register each worker
///  4. *Decider* updated the `lest_seen_block` to the latest seen block from the logs - 1
#[tokio::test]
async fn test_deploy_deals_diff_blocks() {
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const BLOCK_NUMBER_1: u32 = 32;
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    const BLOCK_NUMBER_2: u32 = 33;

    //let counter = Arc::new(Mutex::new(0));
    let (mut server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let mut distro = make_distro_with_api_and_config(url, empty_config);
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
    // Reqs: blockNumber, getLogs, 2x of gasPrice, getTransactionCount and sendRawTransaction
    let expected_reqs_count = 8;
    {
        let mut register_worker_counter = 0;
        for _ in 0..expected_reqs_count {
            let (method, params) = recv_request.recv().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => {
                    json!("0x10")
                }
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    Value::Array(vec![
                        TestApp::log_test_app1(DEAL_ID_1, BLOCK_NUMBER_1, log.topics[1].as_str()),
                        TestApp::log_test_app2(DEAL_ID_2, BLOCK_NUMBER_2, log.topics[1].as_str()),
                    ])
                }
                "eth_sendRawTransaction" => {
                    // TODO: check registered worker_id
                    register_worker_counter += 1;
                    json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
                }
                "eth_getTransactionCount" => json!("0x1"),
                "eth_gasPrice" => json!("0x3b9aca07"),
                _ => panic!("unexpected method: {}", method),
            };
            send_response.send(Ok(response)).unwrap();
        }
        assert_eq!(
            register_worker_counter, 2,
            "expect try register 2 workers for each deal"
        );
    }

    wait_decider_stopped(&mut client).await;

    let (last_seen, deals, mut workers) = {
        let mut result = execute(
            &mut client,
            r#"
            (seq
                (call relay ("decider" "get_string") ["last_seen_block"] last_seen)
                (seq
                    (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
                    (call relay ("worker" "list") [] workers)
                )
            )
        "#,
            "last_seen deals workers",
            hashmap! {},
        )
        .await
        .unwrap();
        let last_seen = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();

        let deals = serde_json::from_value::<StringListValue>(result.remove(0)).unwrap();
        assert_eq!(deals.strings.len(), 2);
        let deals = deals
            .strings
            .iter()
            .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
            .collect::<Vec<_>>();

        let workers = serde_json::from_value::<Vec<String>>(result.remove(0))
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();

        (last_seen, deals, workers)
    };
    // Note that it must not be BLOCK_NUMBER_2 since we save BLOCK_NUMBER_2 - 1
    assert_eq!(last_seen.str, to_hex(BLOCK_NUMBER_1));

    let mut expected = hashmap! {
        deal_id_1 => (TestApp::test_app1(), BLOCK_NUMBER_1),
        deal_id_2 => (TestApp::test_app2(), BLOCK_NUMBER_2),
    };
    for deal in deals {
        let worker_exist = workers.remove(&deal.worker_id);
        assert!(
            worker_exist,
            "worker_id from joined_deals is not in the list of peer workers"
        );
        let result = expected.remove(&deal.deal_id);
        assert!(
            result.is_some(),
            "deal_id from joined_deals is not in the list of expected deals"
        );
        let (app, block) = result.unwrap();

        let cid = get_worker_app_cid(&mut client, &deal.worker_id).await;
        assert_eq!(cid, app.cid, "wrong cid");
        let deal_state = get_deal_state(&mut client, &deal.deal_id).await;
        assert_eq!(deal_state.left_boundary, to_hex(block), "wrong saved state");
    }

    server.shutdown().await;
}

/// Test deal deployment in different *Decider* runs
///
/// Plan
/// 1. *Decider* installs a deal.
///     This process already was checked.
/// 2. *Decider* installs another deal.
///    Check exactly what can be changed from the installation of another deal
///    a. `joined_deals` list contains both deals (check that the list is not overwritten)
///    b. state of the deal (stored by `deal_id`)
///    c. both workers are installed and have correct CIDs
#[tokio::test]
async fn test_deploy_a_deal_in_seq() {
    const BLOCK_INIT: u32 = 1;
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const BLOCK_NUMBER_1: u32 = 32;
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    // This block should be out of range of the first deal (+ 500 from
    const BLOCK_NUMBER_2: u32 = 531;

    let (mut server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let mut distro = make_distro_with_api_and_config(url, empty_config);
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

    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction
    for step in 0..5 {
        let (method, params) = recv_request.recv().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => {
                json!(to_hex(BLOCK_INIT))
            }
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                Value::Array(vec![TestApp::log_test_app1(
                    DEAL_ID_1,
                    BLOCK_NUMBER_1,
                    log.topics[1].as_str(),
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
    wait_decider_stopped(&mut client).await;

    let deals = get_joined_deals(&mut client).await;
    assert!(!deals.is_empty(), "decider didn't join any deal");

    // The second run
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction and getLogs for the old deal
    for step in 0..6 {
        let (method, params) = recv_request.recv().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => {
                json!(to_hex(BLOCK_NUMBER_2))
            }
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                if step == 1 {
                    json!([TestApp::log_test_app2(
                        DEAL_ID_2,
                        BLOCK_NUMBER_2,
                        log.topics[1].as_str(),
                    )])
                } else if step == 5 {
                    json!([])
                } else {
                    panic!("call eth_getLogs on the wrong step {step}");
                }
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
    wait_decider_stopped(&mut client).await;

    let (last_seen, deals, mut workers) = {
        let mut result = execute(
            &mut client,
            r#"
            (seq
                (call relay ("decider" "get_string") ["last_seen_block"] last_seen)
                (seq
                    (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
                    (call relay ("worker" "list") [] workers)
                )
            )
            "#,
            "last_seen deals workers",
            hashmap! {},
        )
        .await
        .unwrap();
        let last_seen = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();

        let deals = serde_json::from_value::<StringListValue>(result.remove(0)).unwrap();
        assert_eq!(deals.strings.len(), 2);
        let deals = deals
            .strings
            .iter()
            .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
            .collect::<Vec<_>>();

        let workers = serde_json::from_value::<Vec<String>>(result.remove(0))
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();

        (last_seen, deals, workers)
    };
    assert_eq!(
        last_seen.str,
        to_hex(BLOCK_NUMBER_2),
        "saved wrong last_seen_block"
    );

    let mut expected = hashmap! {
        deal_id_1 => (TestApp::test_app1(), BLOCK_NUMBER_1),
        deal_id_2 => (TestApp::test_app2(), BLOCK_NUMBER_2),
    };
    for deal in deals {
        let worker_exist = workers.remove(&deal.worker_id);
        assert!(
            worker_exist,
            "worker_id from joined_deals is not in the list of peer workers"
        );
        let result = expected.remove(&deal.deal_id);
        assert!(
            result.is_some(),
            "deal_id from joined_deals is not in the list of expected deals"
        );
        let (app, block) = result.unwrap();

        let cid = get_worker_app_cid(&mut client, &deal.worker_id).await;
        assert_eq!(cid, app.cid, "wrong cid");
        let deal_state = get_deal_state(&mut client, &deal.deal_id).await;
        assert_eq!(deal_state.left_boundary, to_hex(block), "wrong saved state");
    }

    server.shutdown().await;
}

/// Test deploying deals from one block
///
/// 1. *Decider* deploys several deals from a block but don't have time to deploy _all_ of them
///    We can simulate it by returning not all deals on the first run, and on the second add deals to the block
#[tokio::test]
async fn test_deploy_deals_in_one_block() {
    enable_decider_logs();
    const BLOCK_INIT: u32 = 1;
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    const BLOCK_NUMBER: u32 = 32;

    let (mut server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let mut distro = make_distro_with_api_and_config(url, empty_config);
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
    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction
        for _ in 0..5 {
            let (method, params) = recv_request.recv().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => {
                    json!(to_hex(BLOCK_INIT))
                }
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    json!([TestApp::log_test_app1(
                        DEAL_ID_1,
                        BLOCK_NUMBER,
                        log.topics[1].as_str(),
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
    // TODO: detect unexpected jsonrpc requests
    wait_decider_stopped(&mut client).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction and getLogs for the old deal
        for step in 0..6 {
            let (method, params) = recv_request.recv().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => {
                    json!(to_hex(BLOCK_INIT))
                }
                "eth_getLogs" => {
                    if step == 1 {
                        let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                        json!([
                            TestApp::log_test_app1(DEAL_ID_1, BLOCK_NUMBER, log.topics[1].as_str()),
                            TestApp::log_test_app2(DEAL_ID_2, BLOCK_NUMBER, log.topics[1].as_str())
                        ])
                    } else {
                        json!([])
                    }
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

    let (last_seen, deals, mut workers) = {
        let mut result = execute(
            &mut client,
            r#"
            (seq
                (call relay ("decider" "get_string") ["last_seen_block"] last_seen)
                (seq
                    (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
                    (call relay ("worker" "list") [] workers)
                )
            )
        "#,
            "last_seen deals workers",
            hashmap! {},
        )
        .await
        .unwrap();
        let last_seen = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();

        let deals = serde_json::from_value::<StringListValue>(result.remove(0)).unwrap();
        assert_eq!(deals.strings.len(), 2);
        let deals = deals
            .strings
            .iter()
            .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
            .collect::<Vec<_>>();

        let workers = serde_json::from_value::<Vec<String>>(result.remove(0))
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();

        (last_seen, deals, workers)
    };
    // TODO: difficult logic with last_seen_block, not sure on what circumstances it should be
    // incremented and when not
    assert_eq!(last_seen.str, to_hex(BLOCK_NUMBER - 1));

    let mut expected = hashmap! {
        deal_id_1 => (TestApp::test_app1(), BLOCK_NUMBER),
        deal_id_2 => (TestApp::test_app2(), BLOCK_NUMBER),
    };
    for deal in deals {
        let worker_exist = workers.remove(&deal.worker_id);
        assert!(
            worker_exist,
            "worker_id from joined_deals is not in the list of peer workers"
        );
        let result = expected.remove(&deal.deal_id);
        assert!(
            result.is_some(),
            "deal_id from joined_deals is not in the list of expected deals"
        );
        let (app, block) = result.unwrap();

        let cid = get_worker_app_cid(&mut client, &deal.worker_id).await;
        assert_eq!(cid, app.cid, "wrong cid");
        let deal_state = get_deal_state(&mut client, &deal.deal_id).await;
        assert_eq!(deal_state.left_boundary, to_hex(block), "wrong saved state");
    }

    server.shutdown().await;
}

/// Test worker registering scenarios  
///
/// Note that atm *Decider* doesn't process the case when worker registration fails
/// the deal is joined nevertheless
///
/// TODO: implement an important test-case
/// When we have logs [log1 from block 0x10, log2 from block 0x20] and _both_
/// registrations fails, we will reinstall only the deal from block 0x20 since we
/// re-check only the last_seen_block. This test is hard to implement properly atm,
/// but need to remember it when fixing Decider.
///
///
#[tokio::test]
#[should_panic]
async fn test_register_worker_fails() {
    const BLOCK_INIT: u32 = 1;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;
    const BLOCK_NUMBER_LATER: u32 = 200;

    let (mut server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let mut distro = make_distro_with_api_and_config(url, empty_config);
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
    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        let error_value = json!({
            "code": -32000,
            "message": "intentional error",
        });
        // Reqs: blockNumber, getLogs and 2x of one of gasPrice, getTransactionCount and sendRawTransaction
        // (try all to not depend on the order)
        for step in 0..3 {
            let (method, params) = recv_request.recv().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => Ok(json!(to_hex(BLOCK_INIT))),
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    Ok(json!([TestApp::log_test_app2(
                        DEAL_ID,
                        BLOCK_NUMBER,
                        log.topics[1].as_str()
                    )]))
                }
                "eth_sendRawTransaction" => Err(error_value.clone()),
                "eth_getTransactionCount" => Err(error_value.clone()),
                "eth_gasPrice" => Err(error_value.clone()),
                _ => panic!("unexpected method: {}", method),
            };
            send_response.send(response).unwrap();
        }
    }
    wait_decider_stopped(&mut client).await;
    let deals = get_joined_deals(&mut client).await;
    assert!(
        deals.is_empty(),
        "since the registration failed, the deal must not be joined"
    );

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction
        for _ in 0..5 {
            let (method, params) = recv_request.recv().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => json!(to_hex(BLOCK_NUMBER_LATER)),
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    assert!(log.from_block <= BLOCK_NUMBER);
                    assert!(log.to_block <= BLOCK_NUMBER);
                    json!([TestApp::log_test_app1(
                        DEAL_ID,
                        BLOCK_NUMBER,
                        log.topics[1].as_str()
                    ),])
                }
                "eth_sendRawTransaction" => {
                    // TODO: how not to wait for the registration if Decider failed?
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
    let deals = get_joined_deals(&mut client).await;
    assert!(!deals.is_empty(), "the deal must be joined after fail")
}

// Update doesn't work and we don't know how it will work in future
// Maybe, make this test ALWAYS fail to remind that this doesn't work?
#[tokio::test]
async fn test_update_deal() {
    enable_decider_logs();
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
                "eth_blockNumber" => json!("0x10"),
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
            let (method, params) = recv_request.recv().await.unwrap();
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
    enable_decider_logs();
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
                "eth_blockNumber" => json!("0x10"),
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
    };
    #[derive(Serialize)]
    struct RemoveMsg {
        remove: Vec<Worker>,
    };
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
    for step in 0..3 {
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
    let mut distro = make_distro_with_api_and_config(url, empty_config);
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

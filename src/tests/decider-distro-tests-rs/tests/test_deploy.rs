#![feature(async_closure)]

pub mod utils;

use crate::utils::setup::{setup_nox, setup_rpc_deploy_deal};
use created_swarm::fluence_spell_dtos::trigger_config::TriggerConfig;
use created_swarm::fluence_spell_dtos::value::{StringListValue, StringValue, U32Value};
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Duration;
use utils::chain::LogsReq;
use utils::control::{
    update_config, update_decider_script_for_tests, wait_decider_stopped, wait_worker_spell_stopped,
};
use utils::default::{
    default_receipt, DEAL_IDS, DEAL_STATUS_ACTIVE, DEFAULT_POLL_WINDOW_BLOCK_SIZE,
};
use utils::distro::{make_distro_with_api, make_distro_with_api_and_config};
use utils::state::deal::{get_deal_state, get_joined_deals, JoinedDeal};
use utils::state::worker::get_worker_app_cid;
use utils::test_rpc_server::{run_test_server, run_test_server_predefined};
use utils::TestApp;
use utils::*;

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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_deploy_a_deal_single() {
    enable_decider_logs();
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK: u32 = 32;
    const LATEST_BLOCK: u32 = 35;
    let server = run_test_server_predefined(async move |method, params| {
        match method.as_str() {
            "eth_blockNumber" => {
                json!(to_hex(LATEST_BLOCK))
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
            "eth_getTransactionReceipt" => default_receipt(),
            "eth_gasPrice" => json!("0x3b9aca07"),
            "eth_call" => json!(DEAL_STATUS_ACTIVE),
            _ => panic!("mock http got unexpected rpc method: {}", method),
        }
    });

    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();

    wait_decider_stopped(&mut client).await;

    let mut result = execute(
        &mut client,
        r#"
                (call relay ("decider" "get_u32") ["counter"] counter)
        "#,
        "counter",
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
    assert_eq!(counter.value, 1, "decider wasn't run");

    // Analyse joined deals
    let deal = {
        let mut deals = get_joined_deals(&mut client).await;
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
    wait_worker_spell_stopped(&mut client, &deal.worker_id, Duration::from_millis(200)).await;

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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_deploy_deals_diff_blocks() {
    const LATEST_BLOCK: u32 = 35;
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const BLOCK_NUMBER_1: u32 = 32;
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    const BLOCK_NUMBER_2: u32 = 33;

    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Reqs: blockNumber, getLogs, 2x of gasPrice, getTransactionCount and sendRawTransaction, getTransactionReceipt, eth_call
    let expected_reqs_count = 12;
    {
        let mut register_worker_counter = 0;
        for _ in 0..expected_reqs_count {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => {
                    json!(to_hex(LATEST_BLOCK))
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
                "eth_getTransactionReceipt" => json!({"status" : "0x1"}),
                "eth_call" => json!(DEAL_STATUS_ACTIVE),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
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
        assert_eq!(deals.value.len(), 2);
        let deals = deals
            .value
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
        last_seen.value,
        to_hex(LATEST_BLOCK),
        "wrong last_seen block"
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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_deploy_a_deal_in_seq() {
    const LATEST_BLOCK_FIRST_RUN: u32 = 35;
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const BLOCK_NUMBER_1: u32 = 32;
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    // This block should be out of range of the first deal + 500
    const LATEST_BLOCK_SECOND_RUN: u32 = 531;

    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;

    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_deploy_deal(
        &mut server,
        BLOCK_NUMBER_1,
        DEAL_ID_1,
        LATEST_BLOCK_FIRST_RUN,
    )
    .await;
    wait_decider_stopped(&mut client).await;

    let deals = get_joined_deals(&mut client).await;
    assert!(!deals.is_empty(), "decider didn't join any deal");

    // The second run
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction, getLogs and eth_call for the old deal
    for step in 0..10 {
        let (method, params) = server.receive_request().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => {
                json!(to_hex(LATEST_BLOCK_SECOND_RUN))
            }
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                if step == 1 {
                    json!([TestApp::log_test_app2(
                        DEAL_ID_2,
                        LATEST_BLOCK_SECOND_RUN,
                        log.topics[1].as_str(),
                    )])
                } else {
                    json!([])
                }
            }
            "eth_sendRawTransaction" => {
                json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
            }
            "eth_getTransactionCount" => json!("0x1"),
            "eth_getTransactionReceipt" => default_receipt(),
            "eth_gasPrice" => json!("0x3b9aca07"),
            "eth_call" => json!(DEAL_STATUS_ACTIVE),
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(response));
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
        assert_eq!(deals.value.len(), 2);
        let deals = deals
            .value
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
        last_seen.value,
        to_hex(LATEST_BLOCK_SECOND_RUN),
        "saved wrong last_seen_block"
    );

    let mut expected = hashmap! {
        // Deal installed on the first run. `left_boundary` should be updated to
        // block min(BLOCK_NUMBER_1 + 500, latest) + 1
        deal_id_1 => (TestApp::test_app1(), std::cmp::min(BLOCK_NUMBER_1 + DEFAULT_POLL_WINDOW_BLOCK_SIZE, LATEST_BLOCK_SECOND_RUN) + 1),
        deal_id_2 => (TestApp::test_app2(), LATEST_BLOCK_SECOND_RUN),
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
        assert_eq!(
            deal_state.left_boundary,
            to_hex(block),
            "wrong saved state for {}",
            deal.deal_id
        );
    }

    server.shutdown().await;
}

/// Test deploying deals from one block
///
/// 1. *Decider* deploys several deals from a block but don't have time to deploy _all_ of them
///    We can simulate it by returning not all deals on the first run, and on the second add deals to the block
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_deploy_deals_in_one_block() {
    enable_decider_logs();

    const LATEST_BLOCK: u32 = 35;
    const DEAL_ID_1: &'static str = DEAL_IDS[0];
    let deal_id_1 = format!("0x{DEAL_ID_1}");
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    let deal_id_2 = format!("0x{DEAL_ID_2}");
    const DEAL_BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID_1, DEAL_BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs, gasPrice, getTransactionCount and sendRawTransaction, getTransactionReceipt
        // and getLogs for the old deal
        for step in 0..10 {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => {
                    json!(to_hex(LATEST_BLOCK))
                }
                "eth_getLogs" => {
                    if step == 1 {
                        let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                        json!([
                            TestApp::log_test_app1(
                                DEAL_ID_1,
                                DEAL_BLOCK_NUMBER,
                                log.topics[1].as_str()
                            ),
                            TestApp::log_test_app2(
                                DEAL_ID_2,
                                DEAL_BLOCK_NUMBER,
                                log.topics[1].as_str()
                            )
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
                "eth_estimateGas" => json!("0x3b9aca07"),
                "eth_getTransactionReceipt" => default_receipt(),
                "eth_call" => json!(DEAL_STATUS_ACTIVE),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
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
        assert_eq!(deals.value.len(), 2);
        let deals = deals
            .value
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
    assert_eq!(last_seen.value, to_hex(LATEST_BLOCK), "wrong last seen");

    let mut expected = hashmap! {
        // It was installed on the first run, so on the second run the window is updated
        deal_id_1 => (TestApp::test_app1(), std::cmp::min(DEAL_BLOCK_NUMBER + DEFAULT_POLL_WINDOW_BLOCK_SIZE, LATEST_BLOCK) + 1),
        deal_id_2 => (TestApp::test_app2(), DEAL_BLOCK_NUMBER),
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
        assert_eq!(
            deal_state.left_boundary,
            to_hex(block),
            "wrong saved state for {}",
            deal.deal_id
        );
    }

    server.shutdown().await;
}

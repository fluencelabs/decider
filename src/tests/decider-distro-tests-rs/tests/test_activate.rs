#![feature(async_closure)]
#![feature(try_blocks)]

use crate::utils::chain::{DealStatusReq, LogsReq};
use crate::utils::control::{
    update_config, update_decider_script_for_tests, wait_decider_stopped, wait_worker_spell_stopped,
};
use crate::utils::default::{default_receipt, DEAL_IDS, DEAL_STATUS_ACTIVE, DEAL_STATUS_INACTIVE};
use crate::utils::distro::make_distro_with_api;
use crate::utils::oneshot_config;
use crate::utils::setup::{setup_nox, setup_rpc_empty_run_with_status};
use crate::utils::state::deal::get_joined_deals;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;
use crate::utils::*;
use serde_json::json;
use std::time::Duration;
use log_utils::enable_logs;

pub mod utils;

/// Test plan:
/// - Use standard nox setup.
/// - On the first run, deploy a deal to create a worker. The deal is INACTIVE.
/// - On the second run, the deal is ACTIVE, so decider should activate the worker.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_activate() {
    enable_logs();
    //enable_decider_logs();
    const LATEST_BLOCK: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    let deal_address = format!("0x{}", DEAL_ID);
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();

    let expected_reqs = 7;
    for _ in 0..expected_reqs {
        let (method, params) = server.receive_request().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(LATEST_BLOCK)),
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
            "eth_call" => {
                let req = serde_json::from_value::<DealStatusReq>(params[0].clone()).unwrap();
                assert_eq!(req.to, deal_address, "request the status of the wrong deal");
                json!(DEAL_STATUS_INACTIVE)
            }
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(response));
    }
    wait_decider_stopped(&mut client).await;

    // At this point, the worker for the deal should be created but inactive.
    let mut worker = worker::get_worker(&mut client, DEAL_ID).await;
    assert_eq!(worker.len(), 1, "worker for inactive deal must be created");
    let worker_id = worker.remove(0);

    // The deal must considered joined
    let joined = get_joined_deals(&mut client).await;
    assert_eq!(joined.len(), 1, "deal must be joined");
    assert_eq!(joined[0].deal_id, deal_address, "deal must be joined");
    assert_eq!(joined[0].worker_id, worker_id, "deal must be joined");

    let active = worker::is_active(&mut client, &deal_address).await.unwrap();
    assert!(!active, "worker must be inactive");

    // The second run on which the deal is activated
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_empty_run_with_status(
        &mut server,
        LATEST_BLOCK + 10,
        DEAL_STATUS_ACTIVE,
        joined.len(),
    )
    .await
    .unwrap();
    wait_decider_stopped(&mut client).await;

    // Check that the worker is indeed activated
    let active = worker::is_active(&mut client, &deal_address).await.unwrap();
    assert!(active, "worker must be active");

    // Check that the worker spell is created and is run after the worker is activated
    wait_worker_spell_stopped(&mut client, &worker_id, Duration::from_millis(500)).await;

    // The next run on which the deal is deactivated
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_empty_run_with_status(
        &mut server,
        LATEST_BLOCK + 10,
        DEAL_STATUS_INACTIVE,
        joined.len(),
    )
    .await
    .unwrap();
    wait_decider_stopped(&mut client).await;

    // Check that the worker is deactivated
    let active = worker::is_active(&mut client, &deal_address).await.unwrap();
    assert!(!active, "worker must be inactive");

    // The next run on which the deal is activated again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_empty_run_with_status(
        &mut server,
        LATEST_BLOCK + 10,
        DEAL_STATUS_ACTIVE,
        joined.len(),
    )
    .await
    .unwrap();
    wait_decider_stopped(&mut client).await;

    // Check that the worker is deactivated
    let active = worker::is_active(&mut client, &deal_address).await.unwrap();
    assert!(active, "worker must be active");
    server.shutdown().await;
}

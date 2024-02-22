#![feature(async_closure)]

pub mod utils;

use crate::utils::control::wait_worker_spell_stopped;
use crate::utils::setup::{setup_rpc_deploy_deal, setup_rpc_deploy_deals};
use crate::utils::state::worker::{get_worker_app_cid, is_active};
use created_swarm::fluence_spell_dtos::value::{StringValue, U32Value};
use maplit::hashmap;
use serde_json::json;
use std::time::Duration;
use utils::chain::LogsReq;
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE, DEFAULT_POLL_WINDOW_BLOCK_SIZE};
use utils::distro::make_distro_with_api;
use utils::setup::setup_nox;
use utils::state::deal::get_joined_deals;
use utils::test_rpc_server::run_test_server;
use utils::TestApp;
use utils::*;

/// Test deal updates
/// 1. Deploy a deal
/// 2. On the second run, return an update
/// 3. After the run, check:
///    - Worker APP CID is updated
///    - Worker was triggered after the update
///      We can check it by checking counter. Worker spell settings is oneshot for the tests, so the counter must be 2:
///      The first run after the activation after installation, the second run after the update.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_deal() {
    enable_decider_logs();
    const LATEST_BLOCK: u32 = 35;
    const LATEST_BLOCK2: u32 = 1000;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();
    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;
    println!("tmp dir: {:?}", swarm.tmp_dir);

    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy test_app_1
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join only one deal");
    let deal = deals.remove(0);

    // run again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        rpc_block_number!(server, LATEST_BLOCK2);
        // no new deals
        rpc_get_logs_empty!(server);
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
        let response = TestApp::log_test_app2_update(&deal.deal_id);
        server.send_response(Ok(json!([response])));
    }
    // deal status phase
    rpc_deal_status!(server, DEAL_STATUS_ACTIVE);
    // deal removed phase
    rpc_get_logs_empty!(server);
    wait_decider_stopped(&mut client).await;

    let cid = {
        let result = execute(
            &mut client,
            r#"(seq
                (call relay ("op" "noop") [])
                (call worker_id ("worker-spell" "get_string") ["h_worker_def_cid"] cid)
            )"#,
            "cid",
            hashmap! { "worker_id" => json!(deal.worker_id )},
        )
        .await
        .unwrap();
        let result = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
        assert!(!result.absent, "no `worker_def_cid` on worker-spell");
        serde_json::from_str::<String>(&result.value).unwrap()
    };
    let original_app = TestApp::test_app1();
    let new_app = TestApp::test_app2();
    assert_ne!(cid, original_app.cid, "CID must be changed");
    assert_eq!(cid, new_app.cid, "CID must be set to the new app");

    wait_worker_spell_stopped(&mut client, &deal.worker_id, Duration::from_millis(500)).await;
    let counter = execute(
        &mut client,
        r#"(seq
            (call relay ("op" "noop") [])
            (call worker_id ("worker-spell" "get_u32") ["hw_counter"] counter)
        )"#,
        "counter",
        hashmap! { "worker_id" => json!(deal.worker_id )},
    )
    .await
    .unwrap();
    let counter = serde_json::from_value::<U32Value>(counter[0].clone()).unwrap();
    assert!(counter.success);
    assert_eq!(
        counter.value, 2,
        "worker must be triggered twice (by installation and by update)"
    );

    server.shutdown().await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_from_later_blocks() {
    const LATEST_BLOCK_INIT: u32 = 50;
    // should be less than LATEST
    const BLOCK_NUMBER_DEAL: u32 = 40;
    // for the window to move, must be greater than the window size
    const LATEST_BLOCK_FIRST_UPDATE: u32 = LATEST_BLOCK_INIT + DEFAULT_POLL_WINDOW_BLOCK_SIZE;
    // some block number in the expected window
    const LATEST_BLOCK_SECOND_UPDATE: u32 =
        BLOCK_NUMBER_DEAL + DEFAULT_POLL_WINDOW_BLOCK_SIZE + 100;
    const DEAL_ID: &'static str = DEAL_IDS[0];

    let mut server = run_test_server();
    let url = server.url.clone();
    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;

    // Deploy a deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK_INIT, DEAL_ID, BLOCK_NUMBER_DEAL).await;
    wait_decider_stopped(&mut client).await;

    let deals = get_joined_deals(&mut client).await;
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
        let to_block = {
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(
                BLOCK_NUMBER_DEAL, log.from_block,
                "deal update should be polled from the block where the deal was deployed"
            );

            server.send_response(Ok(json!([])));
            log.to_block
        };
        // statuses
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_call");
            server.send_response(Ok(json!(DEAL_STATUS_ACTIVE)));
        }
        // deal removed phase
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }

        to_block
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
        // statuses
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_call");
            server.send_response(Ok(json!(DEAL_STATUS_ACTIVE)));
        }
        // deal removed phase
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
    }
    wait_decider_stopped(&mut client).await;

    server.shutdown().await
}

/// Test that deal are still updated when some RPC responses are errors
///
/// Plan:
/// 1. Deploy 3 deals.
/// 2. On updating, getStatus and a remove event return an error for the second deal,
/// 3. Check that the first and the third deals are updated.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_with_errors() {
    enable_decider_logs();
    const LATEST_BLOCK_1: u32 = 30;
    const LATEST_BLOCK_2: u32 = 40;

    let error = json!({
        "code": -32603,
        "message": "The deal must fail",
    });

    let deals = vec![(DEAL_IDS[0], 10), (DEAL_IDS[1], 20), (DEAL_IDS[2], 25)];
    let deal1 = format!("0x{}", DEAL_IDS[0]);
    let deal2 = format!("0x{}", DEAL_IDS[1]);
    let deal3 = format!("0x{}", DEAL_IDS[2]);

    let expected_cid = hashmap! {
        deal1 => TestApp::test_app2().cid,
        deal2 => TestApp::test_app1().cid, // meaning that the deal was not updated and preserved the old cid
        deal3 => TestApp::test_app2().cid,
    };

    let mut server = run_test_server();
    let url = server.url.clone();
    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;
    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;

    // Deploy the 3 deals
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_deploy_deals(&mut server, LATEST_BLOCK_1, deals).await;
    wait_decider_stopped(&mut client).await;

    // Update the deals, return errors for the second deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        rpc_block_number!(server, LATEST_BLOCK_2);
        // for new deals
        rpc_get_logs_empty!(server);
        // update phase
        // first deal, ok
        rpc_get_logs_exact!(
            server,
            Ok(json!([TestApp::log_test_app2_update(DEAL_IDS[0])]))
        );
        // second deal, error
        rpc_get_logs_exact!(server, Err(error.clone()));
        // third deal, ok
        rpc_get_logs_exact!(
            server,
            Ok(json!([TestApp::log_test_app2_update(DEAL_IDS[2])]))
        );
        // at this point all deals that were really updated are updated

        // Now return statuses
        // first deal, ok
        rpc_deal_status!(server, DEAL_STATUS_ACTIVE);
        // second deal, error
        rpc_deal_status_exact!(server, Err(error.clone()));
        // third deal, ok
        rpc_deal_status!(server, DEAL_STATUS_ACTIVE);

        // Poll remove
        // first deal, ok
        rpc_get_logs_empty!(server);
        // second deal, error
        rpc_get_logs_exact!(server, Err(error.clone()));
        // third deal, ok
        rpc_get_logs_empty!(server);
    }
    wait_decider_stopped(&mut client).await;

    // At this point
    // - all three deals must be joined.
    // - first and third deals must be updated
    // - all deals must remain active
    let joined_deals = get_joined_deals(&mut client).await;
    assert_eq!(
        joined_deals.len(),
        3,
        "decider should join all three deals, actually joined {:?}",
        joined_deals
    );
    for deal in joined_deals {
        let cid = get_worker_app_cid(&mut client, &deal.worker_id).await;
        assert_eq!(
            cid, expected_cid[&deal.deal_id],
            "wrong cid for deal {}",
            deal.deal_id
        );
        let deal_active = is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(deal_active, "deal {} must be active", deal.deal_id);
    }
}

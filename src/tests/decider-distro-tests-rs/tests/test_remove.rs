#![feature(async_closure)]
#![feature(try_blocks)]

pub mod utils;

use connected_client::ConnectedClient;
use utils::test_rpc_server::run_test_server;

use crate::utils::chain::LogsReq;
use crate::utils::setup::{setup_rpc_deploy_deal, setup_rpc_empty_run, setup_swarm};
use eyre::WrapErr;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::json;
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE, DEAL_STATUS_ENDED};
use utils::distro::*;
use utils::setup::setup_nox;
use utils::state::deal::get_joined_deals;
use utils::*;

#[allow(dead_code)]
#[derive(Deserialize)]
struct DealStatusReq {
    data: String,
    to: String,
}

/// Test that decider removes a deal ended by the owner.
/// A deal is ended when it's status is 'ended' (0x2 status code from the chain and `getStatus` call)
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_remove_deal() {
    enable_decider_logs();
    const LATEST_BLOCK: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join exactly one deal");
    let deal = deals.remove(0);

    // run again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    for _step in 0..4 {
        let (method, params) = server.receive_request().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(LATEST_BLOCK)),
            "eth_getLogs" => {
                json!([])
            }
            "eth_call" => {
                let req = serde_json::from_value::<DealStatusReq>(params[0].clone()).unwrap();
                assert_eq!(req.to, deal.deal_id);
                json!(DEAL_STATUS_ENDED)
            }
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(json!(response)));
    }
    wait_decider_stopped(&mut client).await;

    // 1. Check that the deal related worker doesn't exist
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

    // 2. Check that the deal is removed from 'joined_deals'
    let joined_deals = get_joined_deals(&mut client).await;
    assert!(joined_deals.is_empty(), "deal must be removed");

    // 3. Check that the deal state is cleared
    let deal_state_str = spell::get_string(&mut client, "decider", &deal.deal_id)
        .await
        .wrap_err("getting deal state")
        .unwrap();
    assert!(deal_state_str.absent, "deal state must be cleared");

    server.shutdown().await
}

/// Test that decider removes a deal when the deal is removed from the provider
/// In this case, ComputeUnitRemoved event is emitted.
///
/// Plan:
/// - run 2 nodes of different providers (it's encoded in the test which node belong to which provider)
/// - deploy a deal on both of them, check that the deal is marked as joined by both of them
/// - send ComputeUnitRemoved event for one provider
/// - check that the deal is removed from the chosen provider, and remains on the other one
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_remove_deal_from_provider() {
    enable_decider_logs();

    const LATEST_BLOCK: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    let deal_id = format!("0x{}", DEAL_ID);
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let swarms = setup_swarm(distro, 2).await;

    let mut client1 = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();
    update_decider_script_for_tests(&mut client1, swarms[0].tmp_dir.clone()).await;

    let mut client2 = ConnectedClient::connect_with_keypair(
        swarms[1].multiaddr.clone(),
        Some(swarms[1].management_keypair.clone()),
    )
    .await
    .unwrap();
    update_decider_script_for_tests(&mut client2, swarms[1].tmp_dir.clone()).await;

    // Deploy a deal on the first provider
    update_config(&mut client1, &oneshot_config())
        .await
        .unwrap();
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client1).await;
    let joined = get_joined_deals(&mut client1).await;
    assert_eq!(joined.len(), 1, "deal must be joined");
    assert_eq!(joined[0].deal_id, deal_id, "deal must be joined");

    // Deploy a deal on the second provider
    update_config(&mut client2, &oneshot_config())
        .await
        .unwrap();
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client2).await;
    let joined = get_joined_deals(&mut client2).await;
    assert_eq!(joined.len(), 1, "deal must be joined");
    assert_eq!(joined[0].deal_id, deal_id, "deal must be joined");

    // Now send remove event on the first provider
    update_config(&mut client1, &oneshot_config())
        .await
        .unwrap();
    {
        rpc_block_number!(server, LATEST_BLOCK);
        // no new deals
        rpc_get_logs_empty!(server);
        // no updates
        rpc_get_logs_empty!(server);
        rpc_deal_status!(server, DEAL_STATUS_ACTIVE);
        // ComputeUnitRemoved event
        {
            let removed_event = json!([{
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0x2716f70beb0f39a94c6edfd057ee4584fd8b3308effd375bb3942523276e3348",
                "blockHash": "0xa8b883a7e2abee52e7e1c248790b523ee4ce197ae9063e3f09fddb708ef32b4d",
                "blockNumber": "0x2d53",
                "address": "0xeb92a1b5c10ad7bfdcaf23cb7dda9ea062cd07e8",
                "data": "0x53aadfa1d6cd4c8a18f7eb26bd0b83ca10b664845cd72e2dd871f78b2006f5a7",
                "topics": [
                  "0x5abefe0a1fb3d6df34b14e459422791829e024e367c6df8eaf0bf218cf42fb36",
                    // encoded host_id, we don't check it since we poll by it,
                    // so just put here a placeholder
                  "0xb5ecc6c89e9c2add9a9d3b08e7c8ed2155d980e48870b72cfb9c5c16a088ebfb"
                ]
            }]);
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(
                log.topics.len(),
                2,
                "expected two topics: topic and host_id"
            );
            assert!(log.from_block <= LATEST_BLOCK);
            server.send_response(Ok(removed_event));
        }
    }
    wait_decider_stopped(&mut client1).await;

    let joined = get_joined_deals(&mut client1).await;
    assert!(joined.is_empty(), "the deal must be removed: {:?}", joined);

    // Don't send any remove event on the second provider
    update_config(&mut client2, &oneshot_config())
        .await
        .unwrap();
    setup_rpc_empty_run(&mut server, LATEST_BLOCK + 20, 1).await;
    wait_decider_stopped(&mut client2).await;
    let joined = get_joined_deals(&mut client2).await;
    assert_eq!(joined.len(), 1, "the deal must be still joined");
    assert_eq!(joined[0].deal_id, deal_id, "the deal must be still joined");

    server.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_deal_ended_and_removed_from_provider() {
    enable_decider_logs();
    const LATEST_BLOCK: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.config.dir_config.persistent_base_dir).await;

    // Deploy a deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    setup_rpc_deploy_deal(&mut server, LATEST_BLOCK, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;

    let deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join exactly one deal");
    assert_eq!(
        deals[0].deal_id,
        format!("0x{}", DEAL_ID),
        "decider should join exactly one deal"
    );

    // run again with both deal status ENDED and remove from provider event
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        rpc_block_number!(server, LATEST_BLOCK + 10);
        // no new deals
        rpc_get_logs_empty!(server);
        // no updates for the existing deal
        rpc_get_logs_empty!(server);
        // deal status ENDED
        rpc_deal_status!(server, DEAL_STATUS_ENDED);
        // there should be NO remove event poll since all deals are already removed
    }
    wait_decider_stopped(&mut client).await;
    let joined = get_joined_deals(&mut client).await;
    assert!(joined.is_empty(), "decider should remove all deals");
}

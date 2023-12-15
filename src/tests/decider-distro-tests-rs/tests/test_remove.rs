#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(async_fn_in_trait)]

pub mod utils;

use utils::test_rpc_server::run_test_server;

use crate::utils::default::default_status;
use crate::utils::setup::setup_rpc_deploy_deal;
use eyre::WrapErr;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::json;
use utils::chain::LogsReq;
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::default::{default_receipt, DEAL_IDS, DEAL_STATUS_ENDED};
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

#[tokio::test]
async fn test_remove_deal() {
    enable_decider_logs();
    const BLOCK_INIT: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    // Deploy a deal
    setup_rpc_deploy_deal(&mut server, BLOCK_INIT, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;

    let mut deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "decider should join only one deal");
    let deal = deals.remove(0);

    // run again
    update_config(&mut client, &oneshot_config()).await.unwrap();
    for _step in 0..4 {
        let (method, params) = server.receive_request().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(BLOCK_INIT)),
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

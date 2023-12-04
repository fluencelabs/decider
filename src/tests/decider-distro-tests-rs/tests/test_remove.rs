#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(async_fn_in_trait)]

pub mod utils;

use utils::test_rpc_server::run_test_server;

use crate::utils::default::DEFAULT_POLL_WINDOW_BLOCK_SIZE;
use connected_client::ConnectedClient;
use created_swarm::make_swarms_with_cfg;
use eyre::WrapErr;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::UnitValue;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utils::chain::LogsReq;
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::deal::get_joined_deals;
use utils::default::{default_receipt, DEAL_IDS, DEAL_STATUS_ENDED};
use utils::distro::*;
use utils::setup::setup_nox;
use utils::*;

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
    {
        let expected_reqs = 6;
        for _step in 0..expected_reqs {
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
            server.send_response(Ok(json!(response)));
        }
    }
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

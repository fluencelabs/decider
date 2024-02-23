#![feature(async_closure)]

pub mod utils;

use crate::utils::default::default_status;
use created_swarm::fluence_spell_dtos::trigger_config::TriggerConfig;
use serde_json::json;
use utils::chain::{filter_logs, LogsReq};
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::default::{default_receipt, DEAL_IDS, DEAL_STATUS_ACTIVE};
use utils::distro::make_distro_with_api_and_config;
use utils::setup::setup_nox;
use utils::state::deal::{get_failed_deals, get_joined_deals};
use utils::state::subnet::{get_txs, get_txs_statuses};
use utils::test_rpc_server::run_test_server;
use utils::TestApp;
use utils::*;

/// Test worker registering scenarios
///
///
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_register_worker_fails() {
    //enable_decider_logs();
    const LATEST_BLOCK_FIRST_RUN: u32 = 110;

    const DEAL_ID: &'static str = DEAL_IDS[0];
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    const DEAL_ID_3: &'static str = DEAL_IDS[2];
    const BLOCK_NUMBER: u32 = 32;
    const BLOCK_NUMBER_2: u32 = 50;
    const BLOCK_NUMBER_3: u32 = 100;

    let deals_in_blocks = vec![
        (BLOCK_NUMBER, DEAL_ID),
        (BLOCK_NUMBER_2, DEAL_ID_2),
        (BLOCK_NUMBER_3, DEAL_ID_3),
    ];

    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    // Initial run for installing the first deal
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        let error_value = json!({
            "code": -32000,
            "message": "intentional error",
            "data": "0x212312",
        });
        // Reqs: blockNumber, getLogs and 3x of one of gasPrice, estimateGas, getTransactionCount and sendRawTransaction
        // deal 2 should be ok, but deal 1 and deal 3 should fail in sendRawTransaction
        // - 1 blockNumber
        // - 1 getLogs
        // - Deal 1: 1 * (getBlockByNumber + estimateGas + maxPriorityFeePerGas + getTransactionCount + sendRawTransaction)
        // - Deal 2: 1 * (getBlockByNumber + estimateGas + maxPriorityFeePerGas + getTransactionCount + sendRawTransaction + getTransactionReceipt)
        // - Deal 3: 1 * (getBlockByNumber + estimateGas + maxPriorityFeePerGas + getTransactionCount + sendRawTransaction)
        // - 1 + eth_call
        for step in 0..19 {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                // step 0
                "eth_blockNumber" => Ok(json!(to_hex(LATEST_BLOCK_FIRST_RUN))),
                "eth_getLogs" => {
                    // step 1
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    let logs = filter_logs(&deals_in_blocks, &log);
                    let reply = logs
                        .iter()
                        .map(|(block, deal_id)| {
                            TestApp::log_test_app1(deal_id, *block, log.topics[1].as_str())
                        })
                        .collect::<Vec<_>>();
                    Ok(json!(reply))
                }
                // step 2 for deal 1, step 7 for deal 2, step 13 for deal 3
                "eth_getBlockByNumber" => Ok(json!({"baseFeePerGas": "0x3b9aca07"})),
                // step 3 for deal 1, step 8 for deal 2, step 14 for deal 3
                "eth_estimateGas" => Ok(json!("0x3b9aca07")),
                // step 4 for deal 1, step 9 for deal 2, step 15 for deal 3
                "eth_maxPriorityFeePerGas" => Ok(json!("0x3b9aca07")),
                // step 5 for deal 1, step 10 for deal 2, step 16 for deal 3
                "eth_getTransactionCount" => Ok(json!("0x1")),
                // step 6 for deal 1, step 11 for deal 2, step 17 for deal 3
                "eth_sendRawTransaction" => {
                    if step != 11 {
                        Err(error_value.clone())
                    } else {
                        Ok(json!(
                            "0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05"
                        ))
                    }
                }
                // step 12 for deal 2, 1
                "eth_getTransactionReceipt" => Ok(default_receipt()),
                // step 18 for deal 2
                "eth_call" => Ok(default_status()),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(response);
        }
    }
    wait_decider_stopped(&mut client).await;
    let failed = get_failed_deals(&mut client).await;
    assert_eq!(
        failed.len(),
        2,
        "only one deal must be joined: {:?}",
        failed
    );

    let deals = get_joined_deals(&mut client).await;
    assert_eq!(deals.len(), 1, "only one deal must be joined: {:?}", deals);
    server.shutdown().await;
}

/// Test registering worker transactions tracking
///
/// Decider tracks transactions of joined deals and reports when the deal didn't join a subnet.
///
/// In this test we create 3 deals.
/// 1. On first run, all three transactions are sent to the blockchain and in pending state
/// 2. On second run, we send three different statuses of transactions: failed, ok and rpc error.
///    We expect that on failed, the deal goes to failed_deals
///                   on ok, we print the ok message and forget about the transaction
///                   on rpc error, we retry the transaction
/// 3. On the third run, we send ok status to the `retry` transaction and expect that the deal is removed from the queue
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_transaction_tracking() {
    const LATEST_BLOCK: u32 = 110;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const DEAL_ID_2: &'static str = DEAL_IDS[1];
    const DEAL_ID_3: &'static str = DEAL_IDS[2];
    const BLOCK_NUMBER: u32 = 32;
    const BLOCK_NUMBER_2: u32 = 50;
    const BLOCK_NUMBER_3: u32 = 100;
    const BLOCK_NUMBER_LATER: u32 = 200;

    let deals_in_blocks = vec![
        (BLOCK_NUMBER, DEAL_ID),
        (BLOCK_NUMBER_2, DEAL_ID_2),
        (BLOCK_NUMBER_3, DEAL_ID_3),
    ];

    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    // Initial run for installing the first deal
    // Return NULL for getTransactionReceipt to simulate pending transaction
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs and 3x of getBlockByNumber, maxPriorityFeePerGas, estimateGas, getTransactionCount, sendRawTransaction, getTransactionReceipt, eth_call
        for _step in 0..23 {
            let (method, params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => json!(to_hex(LATEST_BLOCK)),
                "eth_getLogs" => {
                    let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                    let logs = filter_logs(&deals_in_blocks, &log);
                    let reply = logs
                        .iter()
                        .map(|(block, deal_id)| {
                            TestApp::log_test_app1(deal_id, *block, log.topics[1].as_str())
                        })
                        .collect::<Vec<_>>();
                    json!(reply)
                }
                "eth_sendRawTransaction" => {
                    json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
                }
                "eth_getTransactionCount" => json!("0x1"),
                "eth_getBlockByNumber" => json!({"baseFeePerGas": "0x3b9aca07"}),
                "eth_maxPriorityFeePerGas" => json!("0x3b9aca07"),
                "eth_estimateGas" => json!("0x3b9aca07"),
                "eth_getTransactionReceipt" => serde_json::Value::Null,
                "eth_call" => json!(DEAL_STATUS_ACTIVE),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
        }
    }
    wait_decider_stopped(&mut client).await;

    let failed = get_failed_deals(&mut client).await;
    assert!(failed.is_empty(), "should be no failed deals");

    let deals = get_joined_deals(&mut client).await;
    assert_eq!(
        deals.len(),
        deals_in_blocks.len(),
        "all deals should be joined, currently joined: {:?}",
        deals
    );

    let txs = get_txs(&mut client).await;
    assert_eq!(
        deals.len(),
        txs.len(),
        "all deals txs should be in the txs list\nCurrently in joined: {:?}\nCurrently in queue: {:?}",
        deals,
        txs,
    );

    let txs_statuses = get_txs_statuses(&mut client).await;
    assert!(
        txs_statuses.is_empty(),
        "no txs status should be known at this stage"
    );

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        let mut receipts = vec![
            Ok(json!({"status": "0x0", "blockNumber": "0x50"})), // failed
            Ok(json!({"status": "0x1", "blockNumber": "0x51"})), // ok
            Err(json!({
                "code": -32000,
                "message": "intentional error",
            })), // rpc error
        ];
        // Reqs: blockNumber, getLogs, 6x getLogs for updates and remove reqs,
        // 3x of eth_getTransactionReceipt, 3x eth_call
        for _step in 0..14 {
            let (method, _params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => Ok(json!(to_hex(BLOCK_NUMBER_LATER))),
                "eth_getLogs" => Ok(json!([])),
                "eth_getTransactionReceipt" => receipts.pop().unwrap(),
                "eth_call" => Ok(json!(DEAL_STATUS_ACTIVE)),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(response);
        }
    }
    wait_decider_stopped(&mut client).await;

    let failed = get_failed_deals(&mut client).await;
    assert_eq!(failed.len(), 1, "should be exactly one failed deal");

    let txs_statuses = get_txs_statuses(&mut client).await;
    assert_eq!(
        txs_statuses.len(),
        2,
        "should be exactly known two txs statuses"
    );

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        // Reqs: blockNumber, getLogs, 6x getLogs for updates and remove reqs,
        // 1x of eth_getTransactionReceipt, eth_call
        for _step in 0..12 {
            let (method, _params) = server.receive_request().await.unwrap();
            let response = match method.as_str() {
                "eth_blockNumber" => json!(to_hex(BLOCK_NUMBER_LATER)),
                "eth_getLogs" => json!([]),
                "eth_getTransactionReceipt" => json!({"status": "0x1", "blockNumber": "0x55"}),
                "eth_call" => json!(DEAL_STATUS_ACTIVE),
                _ => panic!("mock http got an unexpected rpc method: {}", method),
            };
            server.send_response(Ok(response));
        }
    }
    wait_decider_stopped(&mut client).await;

    let failed = get_failed_deals(&mut client).await;
    assert_eq!(failed.len(), 1, "should be exactly one failed deal");

    let txs_statuses = get_txs_statuses(&mut client).await;
    assert_eq!(
        txs_statuses.len(),
        3,
        "should be exactly known two txs statuses"
    );

    server.shutdown().await;
}

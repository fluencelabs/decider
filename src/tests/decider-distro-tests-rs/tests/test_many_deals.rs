#![feature(async_closure)]
#![feature(try_blocks)]

pub mod utils;

use std::str::FromStr;

use created_swarm::{Args, CreatedSwarm, FunctionOutcome};
use futures::future::BoxFuture;
use futures::FutureExt;
use maplit::hashmap;
use serde_json::json;

use crate::utils::chain::{play_chain, random_tx, ChainReplies, Deal, TxReceipt};
use crate::utils::control::{
    run_decider, update_decider_config, update_decider_script_for_tests, update_worker_config,
    wait_decider_stopped, wait_worker_spell_stopped, wait_worker_spell_stopped_after,
};
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE, TX_RECEIPT_STATUS_OK};
use crate::utils::distro::make_distro_stopped;
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::subnet;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;
use crate::utils::{enable_decider_logs, oneshot_config, TestApp};

/// Test Scenario: Installation of Too Many Deals
///
/// Install several deals at the same time
/// To check:
/// - all deals are installed
/// - all workers are created
///
/// TODO: how to check the situation when the decider don't have enough time to process all
///       the deals in one run?
#[tokio::test]
async fn test_install_many() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deals = DEAL_IDS
        .into_iter()
        .map(|id| Deal::ok(id, TestApp::test_app1(), DEAL_STATUS_ACTIVE))
        .collect::<Vec<_>>();
    let tx_hashes = deals.iter().map(|_| random_tx()).collect::<_>();
    let chain_replies = ChainReplies::ok(deals.clone(), tx_hashes);
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert_eq!(joined.len(), deals.len(), "all deals must be installed, expected: {deals:?}, got: {joined:?}");
    for joined_deal in joined.iter() {
        let is_installed = deals.iter().any(|d| d.deal_id == joined_deal.deal_id);
        assert!(is_installed, "deal {} wasn't installed", joined_deal.deal_id);

        let worker_id = worker::get_worker(&mut client, &joined_deal.deal_id).await;
        assert!(!worker_id.is_empty(), "Worker for deal {} must be created", joined_deal.deal_id);
        let is_active = worker::is_active(&mut client, &joined_deal.deal_id).await.unwrap();
        assert!(is_active, "Worker for deal {} must be active", joined_deal.deal_id);
    }
}

/// Test Scenario: Installation after Update and Remove of another deals
///
/// 1. On first run, install several deals
/// 2. On second run, remove some deals and update others
/// 3. On third run, install a new deal
///
/// To check:
/// - On second run, check that one deal is removed, one is updated and the rest are the same
/// - On third run, check that the new deal is installed and the rest are the same
#[tokio::test]
async fn test_install_again() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    // Install deals
    let deals = DEAL_IDS
        .into_iter()
        .take(3)
        .map(|id| Deal::ok(id, TestApp::test_app1(), DEAL_STATUS_ACTIVE))
        .collect::<Vec<_>>();
    let tx_hashes = deals.iter().map(|_| random_tx()).collect::<_>();
    let chain_replies = ChainReplies::ok(deals.clone(), tx_hashes);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Remove `deals[0]`, update `deals[1]`
    // Install DEAL_IDS[4]
}

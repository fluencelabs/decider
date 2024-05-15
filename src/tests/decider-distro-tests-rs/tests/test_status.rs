#![feature(async_closure)]

use crate::utils::{enable_decider_logs, TestApp};
use crate::utils::chain::{ChainReplies, Deal, random_tx};
use crate::utils::control::run_decider;
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE, DEAL_STATUS_ENDED, DEAL_STATUS_INSUFFICIENT_FUNDS, DEAL_STATUS_NOT_ENOUGH_WORKERS};
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;

pub mod utils;

/// Test Scenario: Activate/Deactivate a deal
///
/// 1. Install an inactive deal (NOT_ENOUGH_WORKERS)
///     - Worker must be deactivated
/// 2. Update the deal status to active
///     - Worker must be activated
/// 3. Update the deal status to inactive again
///     - Worker must be deactivated
/// 4. Update the deal status to INSUFFICIENT_FUNDS
///     - Worker must be deactivated
///     - Note: the deal will be removed on one of the next runs when Nox exists the deal
/// 5. Update the deal status to ENDED
///     - Worker must be deactivated
///     - Note: the deal will be removed on one of the next runs when Nox exists the deal
/// 6. Update the deal status to _some unsupported status_
#[tokio::test]
async fn test_deal_status() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    // Install an inactive deal
    let mut deal = Deal::ok(DEAL_IDS[0], TestApp::test_app1(), DEAL_STATUS_NOT_ENOUGH_WORKERS);
    let tx_hash = random_tx();
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![tx_hash]);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Check the status of the installed deal
    let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
    assert!(!is_active, "Deal {} must be inactive", deal.deal_id);

    // Update the deal status to active
    deal.status = Some(DEAL_STATUS_ACTIVE.to_string());
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
    assert!(is_active, "Deal {} must be active", deal.deal_id);

    // Update the deal status to deactivate
    deal.status = Some(DEAL_STATUS_NOT_ENOUGH_WORKERS.to_string());
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
    assert!(!is_active, "Deal {} must be inactive", deal.deal_id);

    // Update the deal status to INSUFFICIENT_FUNDS
    deal.status = Some(DEAL_STATUS_INSUFFICIENT_FUNDS.to_string());
    let chain_replies = ChainReplies::new(vec![], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
    assert!(!is_active, "Deal {} must be inactive", deal.deal_id);

    // Update the deal status to ENDED
    deal.status = Some(DEAL_STATUS_ENDED.to_string());
    let chain_replies = ChainReplies::new(vec![], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
    assert!(!is_active, "Deal {} must be inactive", deal.deal_id);
    server.shutdown().await;
}

/// Test Scenario: Decider finds the deal with the ended status before the deal is installed
///
/// 1. Chain sends info that the deal in INSUFFICIENT_FUNDS status has matched the peer
/// 2. The deal must not be installed.
/// 3. Chain sends info that the deal in INSUFFICIENT_FUNDS status has matched the peer
/// 4. The deal must not be installed.
#[tokio::test]
async fn test_install_ended() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal = Deal::ok(DEAL_IDS[0], TestApp::test_app1(), DEAL_STATUS_INSUFFICIENT_FUNDS);
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert!(joined.is_empty(), "Deal must not be installed, got {joined:?}");

    let deal = Deal::ok(DEAL_IDS[0], TestApp::test_app1(), DEAL_STATUS_ENDED);
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert!(joined.is_empty(), "Deal must not be installed, got {joined:?}");
    server.shutdown().await;
}
#![feature(async_closure)]

use crate::utils::chain::{ChainReplies, Deal, random_tx, TxReceipt};
use crate::utils::control::run_decider;
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE};
use crate::utils::{chain, enable_decider_logs, TestApp};
use crate::utils::setup::setup_nox;
use crate::utils::state::{deal, subnet, worker};
use crate::utils::test_rpc_server::run_test_server;

pub mod utils;

/// Test Scenario: Decider can recover from the failed installation because of chain errors
///
/// Chain Errors:
/// - `ChainConnector.get_deals` returns an error
///
/// 1. Try to install a deal with `get_deals` failure (get_app_cid returns error)
///    - Check that the state is clear and no worker is created
/// 2. Install a deal
///     - Check that the deal is joined and the worker is created
/// 3. Try to install a deal with `get_deals` failure (get_app_cid returns error)
///     - Check that the state didn't change and the only one worker is created
#[tokio::test]
async fn test_failed_get_deals() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];

    let chain_replies = ChainReplies::new(
        vec![Deal::broken(deal_id)],
        vec![],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert!(joined.is_empty(), "No deals should be installed, got: {joined:?}");
        let workers = worker::get_worker_list(&mut client).await;
        assert!(workers.is_empty(), "No workers should be created, got: {workers:?}");
    }

    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        vec![chain::random_tx()],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    let worker_id = {
        let joined = deal::get_joined_deals(&mut client).await;
        assert!(!joined.is_empty(), "The deal {deal_id} must be joined");
        assert_eq!(deal_id, joined[0].deal_id, "wrong deal is joined");

        let mut worker = worker::get_worker(&mut client, &deal_id).await;
        assert!(!worker.is_empty(), "Worker for deal {deal_id} must be created");
        worker.remove(0)
    };

    let chain_replies = ChainReplies::new(
        vec![Deal::broken(deal_id)],
        vec![],
    );
    run_decider(&mut server, &mut client, chain_replies).await;
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert!(!joined.is_empty(), "The deal {deal_id} must be joined");
        assert_eq!(deal_id, joined[0].deal_id, "wrong deal is joined");

        let worker = worker::get_worker(&mut client, &deal_id).await;
        assert!(!worker.is_empty(), "Worker for deal {deal_id} must be created");
        assert_eq!(worker_id, worker[0], "Worker for the deal {deal_id} has changed");
    }
    server.shutdown().await;
}

/// Test Scenario: Decider can recover from the failed installation because of chain errors
///
/// Chain Errors:
/// - `ChainConnector.register_workers` returns an error
///
/// 1. Try to install a deal with `register_workers` failure
///    - The deal is installed and worker is created
///    - The txs list is empty since no txs was sent
///    - The chain error is in the `failed_deals` list
#[tokio::test]
async fn test_failed_register_workers() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];

    let chain_replies = ChainReplies {
        deals: vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        new_deals_tx_hashes: vec![None],
        new_deals_receipts: vec![],
    };
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert!(!joined.is_empty(), "The deal {deal_id} must be joined");
    assert_eq!(deal_id, joined[0].deal_id, "wrong deal is joined");
    let worker_id = worker::get_worker(&mut client, &deal_id).await;
    assert!(!worker_id.is_empty(), "Worker for deal {deal_id} must be created");

    let txs = subnet::get_txs(&mut client).await;
    assert!(txs.is_empty(), "No txs should be registered, got: {txs:?}");

    let failed = deal::get_failed_deals(&mut client).await;
    assert!(!failed.is_empty(), "No failed deals should be registered, got: {failed:?}");
    assert_eq!(failed[0].deal_id, deal_id, "wrong deal failed");

    server.shutdown().await;
}

/// Test Scenario: Decider can recover from the failed installation because of chain errors
///
/// Chain Errors:
/// - `ChainConnector.get_receipts` returns an error
///
/// 1. Install a deal, register the worker, but provide a pending receipt
/// 2. On the next run, provide the failed receipt
///
#[tokio::test]
async fn test_failed_get_receipts() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];
    let tx = random_tx();
    let chain_replies = ChainReplies {
        deals: vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        new_deals_tx_hashes: vec![Some(tx.clone())],
        new_deals_receipts: vec![Some(TxReceipt { tx_hash: tx.clone(), status: "pending".to_string() })],
    };
    run_decider(&mut server, &mut client, chain_replies).await;
    // We joined the deal, but the status is unknown
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert!(!joined.is_empty(), "deal must be joined");
        assert_eq!(joined[0].deal_id, deal_id);

        let worker = worker::get_worker(&mut client, &deal_id).await;
        assert!(!worker.is_empty(), "worker must be created");

        let txs = subnet::get_txs(&mut client).await;
        assert!(!txs.is_empty(), "txs must be registered");
        assert_eq!(txs[0].tx_hash, tx);
        assert_eq!(txs[0].deal_id, deal_id);

        let statuses = subnet::get_txs_statuses(&mut client).await;
        assert!(statuses.is_empty(), "no statuses should be registered, got: {statuses:?}");
    }

    let chain_replies = ChainReplies {
        deals: vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        new_deals_tx_hashes: vec![],
        new_deals_receipts: vec![None],
    };
    run_decider(&mut server, &mut client, chain_replies).await;
    {
        let txs = subnet::get_txs(&mut client).await;
        assert!(!txs.is_empty(), "txs must be registered");
        assert_eq!(txs[0].tx_hash, tx);
        assert_eq!(txs[0].deal_id, deal_id);

        let statuses = subnet::get_txs_statuses(&mut client).await;
        assert!(statuses.is_empty(), "no statuses should be registered, got: {statuses:?}");
    }

    let chain_replies = ChainReplies {
        deals: vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        new_deals_tx_hashes: vec![],
        new_deals_receipts: vec![Some(TxReceipt { tx_hash: tx.clone(), status: "error".to_string() })],
    };
    run_decider(&mut server, &mut client, chain_replies).await;
    {
        let txs = subnet::get_txs(&mut client).await;
        assert!(txs.is_empty(), "txs must be cleared, got {txs:?}");

        let statuses = subnet::get_txs_statuses(&mut client).await;
        assert!(!statuses.is_empty(), "tx status must be saved, got: {statuses:?}");
    }


    server.shutdown().await;
}

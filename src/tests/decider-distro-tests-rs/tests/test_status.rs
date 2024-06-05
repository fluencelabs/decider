#![feature(async_closure)]

use crate::utils::chain::{random_tx, ChainReplies, Deal};
use crate::utils::control::run_decider;
use crate::utils::default::{
    DEAL_IDS, DEAL_STATUS_ACTIVE, DEAL_STATUS_ENDED, DEAL_STATUS_INSUFFICIENT_FUNDS,
    DEAL_STATUS_NOT_ENOUGH_WORKERS,
};
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;
use crate::utils::{enable_decider_logs, spell, TestApp};

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
///
/// Note: on INSUFFICIENT_FUNDS and ENDED statuses Decider just stops the workers and doesn't remove
/// them because the deals with these statuses must be removed from the list of deals by chain, so
/// Decider will remove the deals when they will be gone from the list.
#[tokio::test]
async fn test_deal_status() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    // Install an inactive deal
    let mut deal = Deal::ok(
        DEAL_IDS[0],
        TestApp::test_app1(),
        DEAL_STATUS_NOT_ENOUGH_WORKERS,
    );
    let tx_hash = random_tx();
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![tx_hash]);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Check the status of the installed deal
    {
        let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(!is_active, "Deal {} must be inactive", deal.deal_id);
    }

    // Update the deal status to active
    {
        deal.status = Some(DEAL_STATUS_ACTIVE.to_string());
        let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
        run_decider(&mut server, &mut client, chain_replies).await;

        let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(is_active, "Deal {} must be active", deal.deal_id);
    }

    // Update the deal status to deactivate
    {
        deal.status = Some(DEAL_STATUS_NOT_ENOUGH_WORKERS.to_string());
        let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
        run_decider(&mut server, &mut client, chain_replies).await;

        let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(!is_active, "Deal {} must be inactive", deal.deal_id);
    }

    // Update the deal status to INSUFFICIENT_FUNDS
    {
        deal.status = Some(DEAL_STATUS_INSUFFICIENT_FUNDS.to_string());
        let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
        run_decider(&mut server, &mut client, chain_replies).await;

        let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(!is_active, "Deal {} must be inactive", deal.deal_id);
    }

    // Update the deal status to ENDED
    {
        deal.status = Some(DEAL_STATUS_ENDED.to_string());
        let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
        run_decider(&mut server, &mut client, chain_replies).await;

        let is_active = worker::is_active(&mut client, &deal.deal_id).await.unwrap();
        assert!(!is_active, "Deal {} must be inactive", deal.deal_id);
    }

    server.shutdown().await;
}

/// Test Scenario: Decider finds the deal with the ended status before the deal is installed
///
/// 1. Chain sends info that the deal in INSUFFICIENT_FUNDS status has matched the peer
/// 2. The deal must not be installed.
/// 3. Chain sends info that the deal in ENDED status has matched the peer
/// 4. The deal must not be installed.
#[tokio::test]
async fn test_install_ended() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal = Deal::ok(
        DEAL_IDS[0],
        TestApp::test_app1(),
        DEAL_STATUS_INSUFFICIENT_FUNDS,
    );
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert!(
        joined.is_empty(),
        "Deal must not be installed, got {joined:?}"
    );

    let deal = Deal::ok(DEAL_IDS[0], TestApp::test_app1(), DEAL_STATUS_ENDED);
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let joined = deal::get_joined_deals(&mut client).await;
    assert!(
        joined.is_empty(),
        "Deal must not be installed, got {joined:?}"
    );
    server.shutdown().await;
}

/// Test Scenario: Worker Spell Double Trigger
///
/// Check that the worker-spell isn't run twice.
/// This can occur when the status is changed to active and the App Cid is changed
///
/// Plan:
/// 1. Install a deal.
/// 2. Change status to NOT_ENOUGH_WORKERS
/// 3. Update the deal to ACTIVE + new App CID
///    Checks:
///    - worker is active
///    - worker app cid is new
///    - worker-spell is run only twice (first installation run + second update run)
///
#[tokio::test]
async fn test_worker_spell_double_run() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    // Install a deal
    let mut deal = Deal::ok(
        DEAL_IDS[0],
        TestApp::test_app1(),
        DEAL_STATUS_ACTIVE,
    );
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![random_tx()]);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Deactive the deal
    deal.status = Some(DEAL_STATUS_NOT_ENOUGH_WORKERS.to_string());
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Update the deal with activation
    let app = TestApp::test_app2();
    let expected_app_cid = app.cid.clone();
    deal.status = Some(DEAL_STATUS_ACTIVE.to_string());
    deal.app = Some(app);
    let chain_replies = ChainReplies::new(vec![deal.clone()], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    {
        let is_active = worker::is_active(&mut client, &deal.deal_id).await.expect("can't get worker active status");
        assert!(is_active, "worker must be activated");
        let worker_id = {
            let mut worker_id = worker::get_worker(&mut client, &deal.deal_id).await;
            assert!(!worker_id.is_empty(), "couldn't get worker id, empty string");
            worker_id.remove(0)
        };

        let app_cid = worker::get_worker_app_cid(&mut client, &worker_id).await;
        assert_eq!(expected_app_cid, app_cid, "the app cid should be changed");

        let counter = spell::get_counter_on(&mut client, &worker_id, "worker-spell").await.expect("worker-spell counter failed");
        assert_eq!(2, counter, "worker-spell must be run only twice");
    }


    server.shutdown().await;
}

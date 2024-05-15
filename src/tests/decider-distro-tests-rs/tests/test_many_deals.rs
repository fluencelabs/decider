#![feature(async_closure)]

use crate::utils::{enable_decider_logs, TestApp};
use crate::utils::chain::{ChainReplies, Deal, random_tx};
use crate::utils::control::run_decider;
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE};
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;

pub mod utils;

/// Test Scenario: Install, Update and Remove Many Deals
///
/// 1. On the first run, install several deals at the same time
///     To check:
///     - all deals are installed
///     - all workers are created
/// 2. On the second run, update several deals at the same time
///     To check:
///     - all deals are still installed
///     - all deal app cids are updated
/// 3. On the third run, remove all the deals
///     To check:
///      - all deals are removed
///
/// TODO: how to check the situation when the decider don't have enough time to process all
///       the deals in one run?
#[tokio::test]
async fn test_many() {
    enable_decider_logs();
    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deals = DEAL_IDS
        .into_iter()
        .map(|id| Deal::ok(id, TestApp::test_app1(), DEAL_STATUS_ACTIVE))
        .collect::<Vec<_>>();
    // Install Deals
    let tx_hashes = deals.iter().map(|_| random_tx()).collect::<_>();
    let chain_replies = ChainReplies::new(deals.clone(), tx_hashes);
    run_decider(&mut server, &mut client, chain_replies).await;
    {
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

    // Update Deals
    let updated_deals = deals
        .into_iter()
        .map(|deal| Deal::ok(&deal.deal_id, TestApp::test_app2(), DEAL_STATUS_ACTIVE))
        .collect::<Vec<_>>();
    run_decider(&mut server, &mut client, ChainReplies::new(updated_deals.clone(), vec![])).await;
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert_eq!(joined.len(), updated_deals.len(), "all deals must be installed, expected: {updated_deals:?}, got: {joined:?}");
        for joined_deal in joined.iter() {
            let deal = updated_deals.iter().find(|d| d.deal_id == joined_deal.deal_id);
            assert!(deal.is_some(), "deal {} wasn't installed", joined_deal.deal_id);
            let deal = deal.unwrap();

            let worker_id = worker::get_worker(&mut client, &joined_deal.deal_id).await;
            assert!(!worker_id.is_empty(), "Worker for deal {} must be created", joined_deal.deal_id);
            let is_active = worker::is_active(&mut client, &joined_deal.deal_id).await.unwrap();
            assert!(is_active, "Worker for deal {} must be active", joined_deal.deal_id);

            let app_cid = worker::get_worker_app_cid(&mut client, &worker_id[0]).await;
            assert_eq!(app_cid, deal.app.as_ref().unwrap().cid, "Deal CID wasn't updated");
        }
    }

    // Remove deals
    run_decider(&mut server, &mut client, ChainReplies::new(vec![], vec![])).await;
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert!(joined.is_empty(), "all deals must be removed, got: {joined:?}");

        let worker_list = worker::get_worker_list(&mut client).await;
        assert!(worker_list.is_empty(), "all workers must be removed, got: {worker_list:?}");
    }
    server.shutdown().await;
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
    let chain_replies = ChainReplies::new(deals.clone(), tx_hashes);
    run_decider(&mut server, &mut client, chain_replies).await;

    // Remove `deals[0]`, update `deals[1]`
    let removed_deal = deals[0].clone();
    let mut deals = deals.into_iter().skip(1).collect::<Vec<_>>();
    deals[0].app = Some(TestApp::test_app2());
    let chain_replies = ChainReplies::new(
        deals.clone(),
        vec![],
    );
    run_decider(&mut server, &mut client, chain_replies).await;
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert_eq!(joined.len(), deals.len(), "deal {removed_deal:?} must be removed, got list of deals: {joined:?}");
        for joined_deal in joined.iter() {
            let deal = deals.iter().find(|d| d.deal_id == joined_deal.deal_id);
            assert!(deal.is_some(), "deal {} wasn't installed", joined_deal.deal_id);
            let deal = deal.unwrap();

            let worker_id = worker::get_worker(&mut client, &joined_deal.deal_id).await;
            assert!(!worker_id.is_empty(), "Worker for deal {} must be created", joined_deal.deal_id);
            let is_active = worker::is_active(&mut client, &joined_deal.deal_id).await.unwrap();
            assert!(is_active, "Worker for deal {} must be active", joined_deal.deal_id);

            let app_cid = worker::get_worker_app_cid(&mut client, &worker_id[0]).await;
            assert_eq!(app_cid, deal.app.as_ref().unwrap().cid, "Deal CID wasn't updated");
        }
    }
    // Install DEAL_IDS[4]
    deals.push(Deal::ok(DEAL_IDS[3], TestApp::test_app1(), DEAL_STATUS_ACTIVE));
    let tx_hashes = vec![random_tx()];
    run_decider(&mut server, &mut client, ChainReplies::new(deals.clone(), tx_hashes)).await;
    {
        let joined = deal::get_joined_deals(&mut client).await;
        assert_eq!(joined.len(), deals.len(), "new deal {} must be installed, expected list of deals: {deals:?}, got: {joined:?}", DEAL_IDS[3]);
        for joined_deal in joined.iter() {
            let deal = deals.iter().find(|d| d.deal_id == joined_deal.deal_id);
            assert!(deal.is_some(), "deal {} wasn't installed", joined_deal.deal_id);
            let deal = deal.unwrap();

            let worker_id = worker::get_worker(&mut client, &joined_deal.deal_id).await;
            assert!(!worker_id.is_empty(), "Worker for deal {} must be created", joined_deal.deal_id);
            let is_active = worker::is_active(&mut client, &joined_deal.deal_id).await.unwrap();
            assert!(is_active, "Worker for deal {} must be active", joined_deal.deal_id);

            let app_cid = worker::get_worker_app_cid(&mut client, &worker_id[0]).await;
            assert_eq!(app_cid, deal.app.as_ref().unwrap().cid, "Deal CID wasn't updated");
        }
    }
    server.shutdown().await;
}

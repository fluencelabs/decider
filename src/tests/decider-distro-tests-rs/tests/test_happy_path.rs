#![feature(async_closure)]
#![feature(try_blocks)]

use crate::utils::{enable_decider_logs, oneshot_config, TestApp};
use crate::utils::chain::{ChainReplies, Deal, random_tx};
use crate::utils::control::{
    run_decider, update_worker_config, wait_worker_spell_stopped, wait_worker_spell_stopped_after,
};
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE};
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::subnet;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;

pub mod utils;

/// Test Scenario: Empty Run
///
/// Check that Decider works fine when there are no deals
#[tokio::test]
async fn test_run_empty() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    run_decider(&mut server, &mut client, ChainReplies::default()).await;

    let deals = deal::get_joined_deals(&mut client).await;
    assert!(deals.is_empty(), "no deals must be installed");

    let workers = worker::get_worker_list(&mut client).await;
    assert!(workers.is_empty(), "no workers must be created");
    server.shutdown().await;
}

/// Test Scenario: Installation Happy Path
///
/// To check on the Installation Phase:
/// 1. Nox State:
///     - A Worker for the Deal is created and is resolved by `deal_id`
///     - A Worker is active (due to the sent ACTIVE status)
///     - Worker Spell on the Worker is installed
///     - Worker Spell has a correct App CID in `h_worker_def_cid`
///     - The App is installed
/// 2. Decider State:
///    - Deal is in `joined_deals`
///    - Transaction for Worker Registration is not in `worker_registration_txs` after checking receipts
///    - Tx hash is stored for the deal
///
#[tokio::test]
async fn test_install_happy_path() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];
    let test_app = TestApp::test_app1();
    let deal_status = DEAL_STATUS_ACTIVE;
    let tx_hash = random_tx();

    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, test_app.clone(), deal_status)],
        vec![tx_hash.clone()],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    // Check that the worker is resolved via deal_id
    let worker_id = {
        let mut worker = worker::get_worker(&mut client, &deal_id).await;
        assert_eq!(
            worker.len(),
            1,
            "worker for the deal {} isn't found",
            deal_id
        );
        worker.remove(0)
    };

    // Check Worker
    {
        let worker_active = worker::is_active(&mut client, &deal_id).await;
        assert!(
            worker_active.as_ref().unwrap_or(&false),
            "worker must be active: {:?}",
            worker_active
        );

        let service_list = worker::service_list_on(&mut client, &worker_id).await;
        assert!(
            service_list.is_ok(),
            "can't get list of services on the worker: {service_list:?}"
        );
        let mut service_list = service_list.unwrap();

        let worker_spell_id = service_list
            .iter()
            .position(|s| s.aliases[0] == "worker-spell");
        assert!(
            worker_spell_id.is_some(),
            "worker-spell isn't installed on the worker, service list: {service_list:?}"
        );
        let worker_spell = service_list.remove(worker_spell_id.unwrap());
        assert_eq!(
            worker_spell.owner_id, worker_id,
            "worker must be owner of the worker spell"
        );
    }

    // Now we know, that the worker and the worker spell are okay, so we can wait for the spell to stop
    wait_worker_spell_stopped(&mut client, &worker_id, std::time::Duration::from_secs(10)).await;

    // Nox State:
    {
        let service_list = worker::service_list_on(&mut client, &worker_id).await;
        assert!(
            service_list.is_ok(),
            "can't get list of services on the worker: {service_list:?}"
        );
        let service_list = service_list.unwrap();

        test_app.services_names.iter().for_each(|s| {
            assert!(
                service_list
                    .iter()
                    .any(|service| service.aliases.contains(s)),
                "service {} isn't installed: {service_list:?}",
                s
            );
        });

        // Check that the owner of the services is the worker
        let worker_is_owner = service_list.iter().all(|s| s.owner_id == worker_id);
        assert!(
            worker_is_owner,
            "worker isn't an owner of all the services: {service_list:?}"
        );
    }

    // Decider State:

    // Check that we store the deal in the Decider State
    {
        let deals = deal::get_joined_deals(&mut client).await;
        assert_eq!(deals.len(), 1);
        assert_eq!(deals[0].deal_id, deal_id);

        let deal_tx_hash = deal::get_deal_tx_hash(&mut client, deal_id).await.unwrap();
        assert!(deal_tx_hash.is_some(), "no tx hash stored for the deal");
        assert_eq!(deal_tx_hash.unwrap(), tx_hash, "wrong tx hash is stored for {deal_id}");
    }
    // Check Transaction Status
    {
        let txs_queue_after_checking_receipts = subnet::get_txs(&mut client).await;
        assert!(
            txs_queue_after_checking_receipts.is_empty(),
            "txs queue for getting receipts must be empty: {:?}",
            txs_queue_after_checking_receipts
        );
    }
    server.shutdown().await;
}

/// Test Scenario: Update Happy Path
/// 1. First, install a deal
/// 2. Second, update the deal with a new app cid
///
/// To check on the Update Phase:
/// 1. Nox State:
///     - Worker is still active
///     - Worker Spell has a new App CID in `h_worker_def_cid`
///     - Worker Spell installed the update
/// 2. Decider State:
///    - Don't change: deal is in `joined_deals`
#[tokio::test]
async fn test_update_happy_path() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];
    let test_app = TestApp::test_app1();
    let deal_status = DEAL_STATUS_ACTIVE;
    let tx_hash = random_tx();

    // Run first time to install
    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, test_app.clone(), deal_status)],
        vec![tx_hash.clone()],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    let worker_id = {
        let mut worker = worker::get_worker(&mut client, &deal_id).await;
        assert_eq!(
            worker.len(),
            1,
            "worker for the deal {} isn't found",
            deal_id
        );
        worker.remove(0)
    };
    wait_worker_spell_stopped(&mut client, &worker_id, std::time::Duration::from_secs(10)).await;

    let test_app_updated = TestApp::test_app2();
    // Run second time to update
    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, test_app_updated.clone(), deal_status)],
        vec![],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Run worker-spell to install the update
    update_worker_config(&mut client, &worker_id, &oneshot_config())
        .await
        .unwrap();

    // Check Worker
    {
        let worker_active = worker::is_active(&mut client, &deal_id).await;
        assert!(
            worker_active.as_ref().unwrap_or(&false),
            "worker must be active: {:?}",
            worker_active
        );

        let service_list = worker::service_list_on(&mut client, &worker_id).await;
        assert!(
            service_list.is_ok(),
            "can't get list of services on the worker: {service_list:?}"
        );
        let mut service_list = service_list.unwrap();

        let worker_spell_id = service_list
            .iter()
            .position(|s| s.aliases[0] == "worker-spell");
        assert!(
            worker_spell_id.is_some(),
            "worker-spell isn't installed on the worker, service list: {service_list:?}"
        );
        let worker_spell = service_list.remove(worker_spell_id.unwrap());
        assert_eq!(
            worker_spell.owner_id, worker_id,
            "worker must be owner of the worker spell"
        );
    }

    // Check that Decider put the new app cid to the worker spell
    {
        let worker_app_cid = worker::get_worker_app_cid(&mut client, &worker_id).await;
        assert_eq!(worker_app_cid, test_app_updated.cid);
    }
    wait_worker_spell_stopped_after(
        &mut client,
        &worker_id,
        current_timestamp,
        std::time::Duration::from_secs(20),
    )
        .await;

    // Check that worker spell installed the updated app
    {
        let service_list = worker::service_list_on(&mut client, &worker_id).await;
        assert!(
            service_list.is_ok(),
            "can't get list of services on the worker: {service_list:?}"
        );
        let service_list = service_list.unwrap();
        test_app_updated.services_names.iter().for_each(|s| {
            assert!(
                service_list
                    .iter()
                    .any(|service| service.aliases.contains(s)),
                "service {} isn't installed: {service_list:?}",
                s
            );
        });
    }

    // Decider State
    let joined = deal::get_joined_deals(&mut client).await;
    assert_eq!(joined.len(), 1);
    assert_eq!(joined[0].deal_id, deal_id);
    server.shutdown().await;
}

/// Test Scenario: Remove Happy Path
/// 1. First, install a deal
/// 2. Second, remove the deal from the list of deals
///
/// To check on the Remove Phase:
/// - on Nox:
///     - can't find a worker by the `deal_id`
///     - `worker_id` doesn't exist
/// - on Decider:
///     - no deal in `joined_deals`
///     - no tx stored for the deal
#[tokio::test]
async fn test_remove_happy_path() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let (_swarm, mut client) = setup_nox(url).await;

    let deal_id = DEAL_IDS[0];
    let test_app = TestApp::test_app1();
    let deal_status = DEAL_STATUS_ACTIVE;
    let tx_hash = random_tx();

    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, test_app.clone(), deal_status)],
        vec![tx_hash.clone()],
    );
    run_decider(&mut server, &mut client, chain_replies).await;

    let worker_id = {
        let mut worker = worker::get_worker(&mut client, &deal_id).await;
        assert_eq!(
            worker.len(),
            1,
            "worker for the deal {} isn't found",
            deal_id
        );
        worker.remove(0)
    };

    // Next run, remove the deal from the list
    run_decider(&mut server, &mut client, ChainReplies::default()).await;

    // 1. Deal isn't resolved
    let worker = worker::get_worker(&mut client, &deal_id).await;
    assert!(
        worker.is_empty(),
        "worker for the deal must be removed: {worker:?}"
    );
    // 2. Worker doesn't exist
    let workers = worker::get_worker_list(&mut client).await;
    assert!(
        workers.is_empty(),
        "no workers must be created: {workers:?}, target worker_id {worker_id}"
    );

    let joined_deals = deal::get_joined_deals(&mut client).await;
    assert!(
        joined_deals.is_empty(),
        "no deals must be installed: {joined_deals:?}, target deal_id {deal_id}"
    );
    let tx_hash = deal::get_deal_tx_hash(&mut client, deal_id).await.unwrap();
    assert!(tx_hash.is_none(), "tx_hash for {deal_id} should be cleaned");


    server.shutdown().await;
}

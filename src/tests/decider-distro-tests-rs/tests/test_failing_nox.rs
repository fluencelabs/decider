#![feature(async_closure)]

use std::sync::Arc;

use futures::future::BoxFuture;
use futures::FutureExt;
use maplit::hashmap;
use serde_json::{json, Value};
use tempfile::TempDir;

use created_swarm::fluence_keypair::KeyPair;
use created_swarm::{Args, CreatedSwarm, FunctionOutcome, JError};

use crate::utils::chain::{random_tx, ChainReplies, Deal};
use crate::utils::control::run_decider;
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE, DEAL_STATUS_NOT_ENOUGH_WORKERS};
use crate::utils::setup::{setup_nox_with, stop_nox};
use crate::utils::state::{deal, worker};
use crate::utils::test_rpc_server::run_test_server;
use crate::utils::{enable_decider_logs, TestApp};

pub mod utils;

/// The scenarios we can't yet test:
/// 1. `Worker.create` works, but `Worker.activate` doesn't work.
/// 2. Builtins failures without re-runs (now we need to re-run nox to fix broken bultins).
/// 3. Remove `joined_deals` state and check that it's restored after re-run.
///    Can't do it due to the Spell KV Security system

/// Test Scenario: Installation with the failing Nox
///
/// Failures:
/// - `Worker.create` doesn't work
///
/// Test Plan:
/// 1. Try to install a deal with a broken `Worker.create`
///    - Checks:
///      - No workers are created: we cannot check this since `Worker.list` isn't available.
///      - No deals are marked as joined
/// 2. Try to install a deal with a working `Worker.create`
///    - Checks:
///      - Worker is created
///      - Deal is marked as joined
///
/// Note: due to CreatedSwarms limitations, to make `Worker.create` work again we need to re-run
/// the node.
#[tokio::test]
async fn test_failed_nox_install_create() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let kp = KeyPair::generate_ed25519();
    let tmp_dir = Arc::new(TempDir::new().expect("couldn't create temp dir"));
    let (swarm, mut client) = setup_nox_with(url.clone(), tmp_dir.clone(), kp.clone()).await;

    // Rewrite the Worker builtin to add a broken `Worker.create`
    let replies = WorkerReplies::default();
    add_broken_worker_builtin(&swarm, replies).await;

    let deal_id = DEAL_IDS[0];
    let deal = Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE);
    let chain_replies = ChainReplies::new(vec![deal], vec![]);
    run_decider(&mut server, &mut client, chain_replies).await;

    {
        let joined_deals = deal::get_joined_deals(&mut client).await;
        assert!(
            joined_deals.is_empty(),
            "No deals should be joined, got: {joined_deals:?}"
        );
    }

    stop_nox(swarm).expect("couldn't stop nox");

    let (_swarm, mut client) = setup_nox_with(url, tmp_dir, kp).await;
    let deal = Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE);
    let tx_hash = random_tx();
    let chain_replies = ChainReplies::new(vec![deal], vec![tx_hash]);
    run_decider(&mut server, &mut client, chain_replies).await;
    // Check that the deal was installed
    {
        let worker = worker::get_worker(&mut client, deal_id).await;
        assert!(!worker.is_empty(), "A worker should be created");

        let joined_deals = deal::get_joined_deals(&mut client).await;
        assert!(!joined_deals.is_empty(), "A deal should be joined");
        assert_eq!(joined_deals[0].deal_id, deal_id);
    }

    server.shutdown().await;
}

/// Test Scenario: Update with the failing Nox
///
/// Failures:
/// - `Worker.activate` doesn't work when activating the deal
///
/// Note: `Worker.activate` DOES work on deal installation
///
/// Test Plan:
/// 1. Install a deal.
/// 2. Update the deal status
/// 3. Fix the Worker builtin (by re-running Nox)
/// 4. Check the deal
///    Checks:
///    - The Deal is still inactive.
/// 4. Update the deal status
///    - Deal status is active
///
#[tokio::test]
async fn test_failed_nox_update_activate() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let tmp_dir = Arc::new(TempDir::new().expect("can't create temp"));
    let kp = KeyPair::generate_ed25519();

    let (swarm, mut client) = setup_nox_with(url.clone(), tmp_dir.clone(), kp.clone()).await;
    let deal_id = DEAL_IDS[0];
    let deal = Deal::ok(
        deal_id,
        TestApp::test_app1(),
        DEAL_STATUS_NOT_ENOUGH_WORKERS,
    );
    let chain_replies = ChainReplies::new(vec![deal], vec![random_tx()]);
    run_decider(&mut server, &mut client, chain_replies).await;

    let worker_id = {
        let mut worker = worker::get_worker(&mut client, deal_id).await;
        assert!(
            !worker.is_empty(),
            "couldn't get worker of a deal {deal_id}"
        );
        worker.remove(0)
    };

    let replies = WorkerReplies {
        is_active: Some(false),
        get_worker_id: Some(worker_id),
    };
    add_broken_worker_builtin(&swarm, replies).await;

    // Update the deal
    let app = TestApp::test_app2();
    let deal = Deal::ok(deal_id, app.clone(), DEAL_STATUS_ACTIVE);
    let chain_replies = ChainReplies::new(vec![deal], vec![]);
    run_decider(&mut server, &mut client, chain_replies.clone()).await;

    stop_nox(swarm).expect("couldn't stop nox");

    let (_swarm, mut client) = setup_nox_with(url.clone(), tmp_dir.clone(), kp.clone()).await;
    {
        let is_active = worker::is_active(&mut client, deal_id).await.unwrap();
        assert!(!is_active, "Worker must be inactive");
    }

    run_decider(&mut server, &mut client, chain_replies).await;
    {
        let is_active = worker::is_active(&mut client, deal_id).await.unwrap();
        assert!(is_active, "Worker must be active");

        let mut worker = worker::get_worker(&mut client, deal_id).await;
        let worker_id = worker.remove(0);

        let app_cid = worker::get_worker_app_cid(&mut client, &worker_id).await;
        assert_eq!(app_cid, app.cid, "cid wasn't updated");
    }

    server.shutdown().await;
}

/// Test Scenario: Installation with the failing Nox
///
/// Failures:
/// - `Worker.remove` doesn't work when removing the deal
///
///
/// Test Plan:
/// 1. Install a deal.
/// 2. Try to remove a deal with broken `Worker.remove`
///    - Checks:
///      - Deal is still joined
///      - Worker still exists
/// 3. Fix the Worker builtin.
/// 4. Remove a deal
///    - Checks:
///       - Deal isn't joined
///       - Worker is removed
#[tokio::test]
async fn test_failed_nox_remove_removed() {
    enable_decider_logs();

    let mut server = run_test_server();
    let url = server.url.clone();

    let tmp_dir = Arc::new(TempDir::new().expect("can't create temp"));
    let kp = KeyPair::generate_ed25519();

    let (swarm, mut client) = setup_nox_with(url.clone(), tmp_dir.clone(), kp.clone()).await;
    let deal_id = DEAL_IDS[0];

    // Install a deal
    let chain_replies = ChainReplies::new(
        vec![Deal::ok(deal_id, TestApp::test_app1(), DEAL_STATUS_ACTIVE)],
        vec![random_tx()],
    );
    run_decider(&mut server, &mut client, chain_replies).await;
    let worker_id = {
        let mut worker = worker::get_worker(&mut client, deal_id).await;
        assert!(!worker.is_empty(), "no worker must be removed");
        worker.remove(0)
    };

    // Break remove
    let replies = WorkerReplies {
        is_active: None,
        get_worker_id: Some(worker_id),
    };
    add_broken_worker_builtin(&swarm, replies).await;

    // Try to remove the deal
    run_decider(&mut server, &mut client, ChainReplies::default()).await;
    {
        let joined_deals = deal::get_joined_deals(&mut client).await;
        assert!(!joined_deals.is_empty(), "deal must be still joined");
        let worker = worker::get_worker(&mut client, deal_id).await;
        assert!(!worker.is_empty(), "no worker must be removed");
    }
    stop_nox(swarm).expect("can't stop nox");
    let (_swarm, mut client) = setup_nox_with(url.clone(), tmp_dir.clone(), kp.clone()).await;
    run_decider(&mut server, &mut client, ChainReplies::default()).await;
    {
        let joined_deals = deal::get_joined_deals(&mut client).await;
        assert!(joined_deals.is_empty(), "deal must not be joined");
        let worker = worker::get_worker(&mut client, deal_id).await;
        assert!(worker.is_empty(), "worker must be removed");
    }
}

#[derive(Default)]
struct WorkerReplies {
    is_active: Option<bool>,
    get_worker_id: Option<String>,
}

async fn add_broken_worker_builtin<'a>(swarm: &CreatedSwarm, replies: WorkerReplies) {
    let err = |msg: String| -> Box<
        dyn Fn(_, _) -> BoxFuture<'static, FunctionOutcome> + 'static + Send + Sync,
    > {
        Box::new(move |_, _| {
            let msg = msg.clone();
            async move { FunctionOutcome::Err(JError::new(msg)) }.boxed()
        })
    };

    let none =
        || -> Box<dyn Fn(_, _) -> BoxFuture<'static, FunctionOutcome> + 'static + Send + Sync> {
            Box::new(move |_, _| async move { FunctionOutcome::Ok(Value::Array(vec![])) }.boxed())
        };

    let constant = |val: Value| -> Box<
        dyn Fn(_, _) -> BoxFuture<'static, FunctionOutcome> + 'static + Send + Sync,
    > {
        Box::new(move |_, _| {
            let val = val.clone();
            async move { FunctionOutcome::Ok(val) }.boxed()
        })
    };
    let is_active = if let Some(is_active) = replies.is_active {
        constant(json!(is_active))
    } else {
        none()
    };
    let get_worker_id = if let Some(worker_id) = replies.get_worker_id {
        constant(json!(vec![worker_id]))
    } else {
        none()
    };
    swarm
        .aquamarine_api
        .clone()
        .add_service(
            "worker".into(),
            hashmap! {
                "create".to_string() => err("Worker creation failed".to_string()).into(),
                "get_worker_id".to_string() => get_worker_id.into(),
                "activate".to_string() => err("Worker activation failed".to_string()).into(),
                "is_active".to_string() => is_active.into(),
                "remove".to_string() => err("Worker removal failed".to_string()).into(),
            },
        )
        .await
        .expect("add service");
}

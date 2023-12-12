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
use maplit::hashmap;
use serde_json::{json, Value};
use utils::chain::LogsReq;
use utils::control::{update_config, update_decider_script_for_tests, wait_decider_stopped};
use utils::distro::*;
use utils::setup::setup_nox;
use utils::*;

#[test]
fn test_connector_config_check() {
    let connector = decider_distro::connector_service_modules();
    let marine_config: Result<TomlMarineConfig, _> = toml::from_slice(connector.config);
    assert!(
        marine_config.is_ok(),
        "connector marine config is not valid toml"
    );
}

#[tokio::test]
async fn test_decider_installed() {
    let distro = make_distro_default();
    let names = package_items_names(&distro);
    assert_eq!(
        names.len(),
        2,
        "expect only 2 services and spells in the decider package"
    );

    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        // disable built-in system services (disabled by default for now, but just in case)
        cfg.enabled_system_services = vec![];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;
    let mut client = ConnectedClient::connect_to(swarms[0].multiaddr.clone())
        .await
        .unwrap();

    let result = client
        .execute_particle(
            r#"
        (seq
          (call relay ("srv" "list") [] list)
          (call client ("return" "") [list])
        )
       "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string())
            },
        )
        .await
        .unwrap();

    if let [Value::Object(service1), Value::Object(service2)] =
        result[0].as_array().expect("expect an array").as_slice()
    {
        let alias1 = service1["aliases"].as_array().unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        let alias2 = service2["aliases"].as_array().unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        assert!(names.contains(&alias1), "{alias1} service is not installed");
        assert!(names.contains(&alias2), "{alias2} service is not installed");
    }
}

///
/// Test how *Decider* calculates the block to poll.
/// Block numbers sequence:
/// 1. `0x0`  -- Decider should be able to poll from the beginning and don't break,so
///            saved last_seen_block should be `0x0`
///            In the eth_getLogs request: we can check that fromBlock is `0x0`
///
/// 2. `0x10` -- the number is less then the range decider polls
///            Decider should move it's left boundary to this block, so
///            saved last_seen_block should be `0x10`
///            In the eth_getLogs request: we can check that fromBlock is `0x1`
///
/// 3. `0x10` -- again the same number in not very realistic case when Decider is too fast and the chain is too slow
///            Decider shouldn't move it's left boundary anywhere, so
///            saved last_seen_block should be `0x10`
///            In the eth_getLogs request: we can check that fromBlock is `0x11`
///
/// 4. `0xffffff` -- big number which Decider shouldn't be able to process during one iteration
///            Decider should move to some boundary configured boundary, which is 500 blocks, so
///            saved last_seen_block should be `0x205`
///            In the eth_getLogs request: we can check that fromBlock is `0x11`
///            Note: the test depend on the knowledge that the range is 500 blocks,
///                  we don't evaluate the number automatically
///
#[tokio::test]
async fn test_left_boundary_idle() {
    let mut server = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let distro = make_distro_with_api_and_config(url, empty_config);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    // To be able to wait 'til the end of one cycle
    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;

    let block_numbers = vec!["0x0", "0x10", "0x10", "0xffffff"];
    let expected_last_seen = vec!["0x0", "0x10", "0x10", "0x205"];
    let expected_from_blocks = vec!["0x0", "0x1", "0x11", "0x11"];

    for step in 0..block_numbers.len() {
        update_config(&mut client, &oneshot_config()).await.unwrap();
        {
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            assert!(params.is_empty());
            server.send_response(Ok(json!(block_numbers[step])));
        }

        {
            let (method, params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            assert!(!params.is_empty());
            let log_req = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(to_hex(log_req.from_block), expected_from_blocks[step]);

            server.send_response(Ok(json!([])));
        }
        wait_decider_stopped(&mut client).await;

        let last_seen = spell::get_string(&mut client, "decider", "last_seen_block")
            .await
            .unwrap();
        assert_eq!(last_seen.str, expected_last_seen[step]);
    }

    server.shutdown().await;
}

#[tokio::test]
async fn test_sync_info() {
    const LATEST_BLOCK_FIRST_RUN: u32 = 100;
    const LATEST_BLOCK_SECOND_RUN: u32 = 2000;

    let mut server = run_test_server();

    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            server.send_response(Ok(json!(to_hex(LATEST_BLOCK_FIRST_RUN))));
        }
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
    }
    wait_decider_stopped(&mut client).await;

    let sync_info = decider::get_sync_info(&mut client)
        .await
        .wrap_err("get_sync_info")
        .unwrap();
    assert_eq!(
        sync_info.run_updated, 1,
        "should be updated on the first run"
    );
    assert_eq!(sync_info.blocks_diff, 0, "must be in sync");

    update_config(&mut client, &oneshot_config()).await.unwrap();
    {
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            server.send_response(Ok(json!(to_hex(LATEST_BLOCK_SECOND_RUN))));
        }
        {
            let (method, _params) = server.receive_request().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            server.send_response(Ok(json!([])));
        }
    }
    wait_decider_stopped(&mut client).await;

    let sync_info = decider::get_sync_info(&mut client)
        .await
        .wrap_err("get_sync_info")
        .unwrap();
    assert_eq!(
        sync_info.run_updated, 2,
        "should be updated on the first run"
    );

    let expected_last_seen = LATEST_BLOCK_FIRST_RUN + DEFAULT_POLL_WINDOW_BLOCK_SIZE;
    assert_eq!(
        sync_info.blocks_diff,
        LATEST_BLOCK_SECOND_RUN - expected_last_seen - 1,
        "must be not in sync with {} block range",
        DEFAULT_POLL_WINDOW_BLOCK_SIZE
    );

    server.shutdown().await;
}

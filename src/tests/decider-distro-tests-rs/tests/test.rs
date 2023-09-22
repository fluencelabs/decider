#![feature(async_closure)]
#![allow(warnings)]
#![feature(try_blocks)]

use crate::test_rpc_server::{run_test_server, run_test_server_predefined};
use connected_client::ConnectedClient;
use created_swarm::make_swarms_with_cfg;
use decider_distro::DeciderConfig;
use eyre::WrapErr;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::{StringListValue, StringValue, U32Value};
use hyper::body::Buf;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::future::Future;
use std::io::Read;
use std::sync::Arc;
use system_services::{PackageDistro, ServiceDistro, SpellDistro};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tracing::log::Log;

mod test_rpc_server;

fn package_items_names(distro: &PackageDistro) -> Vec<String> {
    distro
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(distro.spells.iter().map(|s| s.name.clone()))
        .collect()
}

// TODO: read config from some config file
pub fn make_distro(trigger_config: TriggerConfig, settings: DeciderConfig) -> PackageDistro {
    let connector = decider_distro::connector_service_modules();
    let marine_config: TomlMarineConfig =
        toml::from_slice(connector.config).expect("parse marine config");
    let service = ServiceDistro {
        modules: connector.modules,
        config: marine_config,
        name: connector.name.to_string(),
    };

    let distro_spell = decider_distro::decider_spell(settings);
    let spell = SpellDistro {
        name: "decider".to_string(),
        air: distro_spell.air.clone(),
        kv: distro_spell.kv.clone(),
        trigger_config,
    };

    PackageDistro {
        name: "decider".to_string(),
        version: decider_distro::VERSION,
        services: vec![service],
        spells: vec![spell],
        init: None,
    }
}

pub fn make_distro_default() -> PackageDistro {
    let decider_settings = DeciderConfig {
        worker_period_sec: 0,
        worker_ipfs_multiaddr: "/ip4/127.0.0.1/tcp/5001".to_string(),
        chain_api_endpoint: "http://127.0.0.1:12009".to_string(),
        chain_network_id: 11,
        chain_contract_block_hex: "0x0".to_string(),
        chain_matcher_addr: "0x0".to_string(),
        chain_workers_gas: 210_00,
        chain_wallet_key: "0x0".to_string(),
    };
    // let's try to run a decider cycle on demand by updating the config
    let mut trigger_config = TriggerConfig::default();
    trigger_config.clock.start_sec = 1;
    make_distro(trigger_config, decider_settings)
}

pub fn make_distro_with_api(api: String) -> PackageDistro {
    let decider_settings = DeciderConfig {
        // worker will run once
        worker_period_sec: 0,
        worker_ipfs_multiaddr: "/ip4/127.0.0.1/tcp/5001".to_string(),
        chain_api_endpoint: api,
        chain_network_id: 11,
        chain_contract_block_hex: "0x0".to_string(),
        chain_matcher_addr: "0x0".to_string(),
        chain_workers_gas: 210_00,
        chain_wallet_key: "0xfdc4ba94809c7930fe4676b7d845cbf8fa5c1beae8744d959530e5073004cf3f"
            .to_string(),
    };
    // decider will run once
    let mut trigger_config = TriggerConfig::default();
    trigger_config.clock.start_sec = 1;
    make_distro(trigger_config, decider_settings)
}

pub fn make_distro_with_api_and_config(api: String, config: TriggerConfig) -> PackageDistro {
    let decider_settings = DeciderConfig {
        // worker will run once
        worker_period_sec: 0,
        worker_ipfs_multiaddr: "/ip4/127.0.0.1/tcp/5001".to_string(),
        chain_api_endpoint: api,
        chain_network_id: 11,
        chain_contract_block_hex: "0x0".to_string(),
        chain_matcher_addr: "0x0".to_string(),
        chain_workers_gas: 210_00,
        chain_wallet_key: "0xfdc4ba94809c7930fe4676b7d845cbf8fa5c1beae8744d959530e5073004cf3f"
            .to_string(),
    };
    // decider will run once
    make_distro(config, decider_settings)
}

async fn execute(
    client: &mut ConnectedClient,
    correct_air: &str,
    return_values: &str,
    mut data: HashMap<&str, Value>,
) -> eyre::Result<Vec<Value>> {
    data.insert("relay", json!(client.node.to_string()));
    data.insert("client", json!(client.peer_id.to_string()));

    client
        .execute_particle(
            format!("(seq {correct_air} (call client (\"return\" \"\") [{return_values}]) )"),
            data,
        )
        .await
}

async fn update_config(
    client: &mut ConnectedClient,
    trigger_config: &TriggerConfig,
) -> eyre::Result<Vec<Value>> {
    execute(
        client,
        r#"(call relay ("spell" "update_trigger_config") ["decider" config])"#,
        "\"done\"",
        hashmap! {
            "config" => json!(trigger_config),
        },
    )
    .await
}

fn enable_decider_logs() {
    let namespaces = vec![
        "run-console=debug",
        // "spell_event_bus=trace",
        //"system_services=debug", //"ipfs_effector=debug",
        //ipfs_pure=debug",
        //"aquamarine::log=debug",
    ];

    let namespaces = namespaces
        .into_iter()
        .map(|ns| {
            ns.trim()
                .parse()
                .unwrap_or_else(|e| panic!("cannot parse {ns} to Directive: {e}"))
        })
        .collect();
    let spec = log_utils::LogSpec::new(namespaces).with_level(tracing::metadata::Level::ERROR);
    log_utils::enable_logs_for(spec);
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogsReq {
    from_block: String,
    to_block: String,
    topics: Vec<String>,
}

struct TestApp {
    cid: String,
    services_names: Vec<String>,
}

impl TestApp {
    // Predefined url_downloader app
    fn test_app1() -> Self {
        Self {
            cid: "bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m".to_string(),
            services_names: vec!["url_downloader".to_string()],
        }
    }

    fn log_test_app1(log: LogsReq) -> Value {
        // Encoded CID (url-downloader): bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m
        // TODO: generate this on fly
        json!([
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x1",
                    "blockHash": "0x2",
                    "blockNumber": log.to_block,
                    "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                    "data": "0x000000000000000000000000ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b9900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3",
                    "topics": [
                        "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                        log.topics[1]
                    ]
                },
            ]
        )
    }
}

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

/// Required IPFS node
#[tokio::test]
async fn test_deploy_deal() {
    //enable_decider_logs();

    let mut server = test_rpc_server::run_test_server_predefined(async move |method, params| {
        match method.as_str() {
            "eth_blockNumber" => {
                json!("0x10")
            }
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                TestApp::log_test_app1(log)
            }
            "eth_sendRawTransaction" => {
                json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
            }
            "eth_getTransactionCount" => json!("0x1"),
            "eth_gasPrice" => json!("0x3b9aca07"),
            _ => panic!("unexpected method: {}", method),
        }
    });

    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;

    let mut client = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();

    // how to wait until decider is over?
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let mut result = execute(
        &mut client,
        r#"
            (seq
                (call relay ("decider" "get_u32") ["counter"] counter)
                (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
            )
        "#,
        "counter deals",
        hashmap! {},
    )
    .await
    .unwrap();

    let counter_value = result.remove(0);
    let counter = serde_json::from_value::<U32Value>(counter_value).unwrap();

    // Here we check that decider was really run. Maybe remove it when I figure out how to
    assert!(
        !counter.absent,
        "decider hasn't started yet (no counter in kv)"
    );
    assert_eq!(counter.num, 1, "decider wasn't run");

    // Analyse joined deals
    let deals_value = result.remove(0);
    let deals = serde_json::from_value::<StringListValue>(deals_value).unwrap();

    // 1. Check that we joined a deal
    assert!(!deals.strings.is_empty(), "decider didn't join any deals");

    #[derive(Deserialize, Debug)]
    struct JoinedDeal {
        deal_id: String,
        worker_id: String,
    }
    let deal = serde_json::from_str::<JoinedDeal>(&deals.strings[0]).unwrap();

    // 2. Check that we can find worker_id by deal_id
    let result = execute(
        &mut client,
        r#"(call relay ("worker" "get_worker_id") [deal_id] resolved_worker_id)"#,
        "resolved_worker_id",
        hashmap! {
            "deal_id" => json!(deal.deal_id),
        },
    )
    .await
    .unwrap();
    let result = result[0]
        .as_array()
        .expect("worker.get_worker_id is not array");
    assert!(!result.is_empty(), "can't resolve worker_id by deal_id");
    let resolved_worker_id = result[0].as_str().expect("worker_id isn't str");
    assert_eq!(deal.worker_id, resolved_worker_id);

    // 3. Check that we can see the list of services on the worker

    #[derive(Deserialize, Debug)]
    struct ServiceInfo {
        aliases: Vec<String>,
        worker_id: String,
        service_type: String,
    }

    let result = execute(
        &mut client,
        r#"
            (seq
                (call relay ("op" "noop") [])
                (call worker_id ("srv" "list") [] list)
            )
        "#,
        "list",
        hashmap! {
            "worker_id" => json!(deal.worker_id),
        },
    )
    .await
    .unwrap();

    // Here we also test that the Installation Spell worked correctly to ensure that the distro is fine,
    // but deep Installation Spell testing is out of scope of this test suits
    let result = serde_json::from_value::<Vec<ServiceInfo>>(result[0].clone()).unwrap();
    let test_app_1 = TestApp::test_app1();
    let worker_spell = result
        .iter()
        .find(|info| info.aliases.contains(&"worker-spell".to_string()));
    assert!(worker_spell.is_some(), "no worker-spell on the worker");
    let worker_spell = worker_spell.unwrap();
    assert_eq!(
        worker_spell.service_type, "spell",
        "worker-spell is not a spell"
    );
    assert_eq!(
        worker_spell.worker_id, deal.worker_id,
        "worker-spell has different worker_id"
    );

    let test_service = result
        .iter()
        .find(|info| info.aliases.contains(&test_app_1.services_names[0]));
    assert!(
        test_service.is_some(),
        "no test service on the worker from a deal"
    );
    let test_service = test_service.unwrap();
    assert_eq!(
        test_service.service_type, "service",
        "test service is not a service"
    );
    assert_eq!(
        test_service.worker_id, deal.worker_id,
        "test service has different worker_id"
    );

    // 4. Check that the worker-spell has the same CID as we wanted to deploy
    // We do it since it's Decider's responsibility to set the correct CID
    let mut result = execute(
        &mut client,
        r#"
        (seq
            (call relay ("op" "noop") [])
            (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
        )
        "#,
        "cid",
        hashmap! {
            "worker_id" => json!(deal.worker_id),
        },
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");

    server.shutdown().await
}

///
/// Test how Decider calculates the block to poll.
/// Block numbers sequence:
/// 1. 0x0  -- Decider should be able to poll from the beginning and don't break,so
///            saved last_seen_block should be 0x0
///            In the eth_getLogs request: we can check that fromBlock is 0x0
///
/// 2. 0x10 -- the number is less then the range decider polls
///            Decider should move it's left boundary to this block, so
///            saved last_seen_block should be 0x10
///            In the eth_getLogs request: we can check that fromBlock is 0x1
///
/// 3. 0x10 -- again the same number in not very realistic case when Decider is too fast and the chain is too slow
///            Decider shouldn't move it's left boundary anywhere, so
///            saved last_seen_block should be 0x10
///            In the eth_getLogs request: we can check that fromBlock is 0x11
///
/// 4. 0xffffff -- big number which Decider shouldn't be able to process during one iteration
///            Decider should move to some boundary configured boundary, which is 500 blocks, so
///            saved last_seen_block should be 0x205
///            In the eth_getLogs request: we can check that fromBlock is 0x11
///            Note: the test depend on the knowledge that the range is 500 blocks,
///                  we don't evaluate the number automatically
///
#[tokio::test]
async fn test_left_boundary_idle() {
    //enable_decider_logs();

    let (server, mut recv_request, send_response) = run_test_server();
    let url = server.url.clone();

    let empty_config = TriggerConfig::default();
    let mut distro = make_distro_with_api_and_config(url, empty_config);
    let swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        cfg
    })
    .await;
    let mut client = ConnectedClient::connect_with_keypair(
        swarms[0].multiaddr.clone(),
        Some(swarms[0].management_keypair.clone()),
    )
    .await
    .unwrap();

    let mut oneshot_config = TriggerConfig::default();
    oneshot_config.clock.start_sec = 1;

    let block_numbers = vec!["0x0", "0x10", "0x10", "0xffffff"];
    let expected_last_seen = vec!["0x0", "0x10", "0x10", "0x205"];
    let expected_from_blocks = vec!["0x0", "0x1", "0x11", "0x11"];

    for step in 0..block_numbers.len() {
        update_config(&mut client, &oneshot_config).await.unwrap();
        {
            let (method, params) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_blockNumber");
            assert!(params.is_empty());
            send_response.send(json!(block_numbers[step])).unwrap();
        }

        {
            let (method, params) = recv_request.recv().await.unwrap();
            assert_eq!(method, "eth_getLogs");
            assert!(!params.is_empty());
            let log_req = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            assert_eq!(log_req.from_block, expected_from_blocks[step]);

            send_response.send(json!([])).unwrap();
        }
        // TODO: change to waiting the signal from decider
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let result = execute(
            &mut client,
            r#" (call relay ("decider" "get_string") ["last_seen_block"] last_seen) "#,
            "last_seen",
            hashmap! {},
        )
        .await
        .unwrap();
        let last_seen = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
        assert_eq!(last_seen.str, expected_last_seen[step]);
    }

    server.shutdown().await;
}

#![feature(async_closure)]
#![allow(warnings)]
#![feature(try_blocks)]

use connected_client::ConnectedClient;
use created_swarm::make_swarms_with_cfg;
use decider_distro::DeciderConfig;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::{ScriptValue, StringListValue, StringValue, U32Value, UnitValue};
use hyper::body::Buf;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response};
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use system_services::{PackageDistro, ServiceDistro, SpellDistro};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task;
use tracing::log::Log;

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

// Can't do it right now since the manager can't update the script
async fn update_script_template(client: &mut ConnectedClient) {
    let script = execute(
        client,
        r#"(call relay ("decider" "get_script_source_from_file") [] script)"#,
        "script",
        hashmap! {},
    )
    .await
    .unwrap();
    let script = serde_json::from_value::<ScriptValue>(script[0].clone()).unwrap();
    assert!(
        script.success,
        "get_script_source_from_file failed with error {}",
        script.error
    );
    let updated_script = format!(
        r#"(seq ({}) (seq (call %init_peer_id% ("run-console" "print") ["return particle"]) (call {} ("return" "") ["done"])))"#,
        script.source_code, client.peer_id,
    );
    let updated_script = r#"(call %init_peer_id% ("run-console" "print") ["hi"])"#;
    let result = execute(
        client,
        r#"(call relay ("decider" "set_script_source_to_file") [script] result)"#,
        "result",
        hashmap! {
            "script" => json!(updated_script),
        },
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<UnitValue>(result[0].clone()).unwrap();
    assert!(result.success, "couldn't update the script");
}

/*
fn inject_decider_spell(pkg: &mut PackageDistro) {
    let mut decider = pkg.spells.iter().find(|s| s.name == "decider").unwrap();
    let script = decider.air;
    let updated_script = format!(
        r#"(seq
            (call %init_peer_id% ("run-console" "print") ["tipa decider"])
            (seq
                (call %init_peer_id% ("run-console" "print") ["return particle"])
                (seq
                    (call %init_peer_id% ("decider" "get_string") ["client_peer_id"] client_peer_id)
                    (call client_peer_id.str ("return" "") ["done"])
                )
             )
        )"#,
    );
    decider.air = updated_script;
}

async fn store_client_id(client: &mut ConnectedClient) {
    let peer_id = client.peer_id.to_string();
    let result = execute(
        client,
        &format!(r#"(call relay ("decider" "set_string") ["{peer_id}"] result)"#),
        "result",
        hashmap! {},
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<UnitValue>(result[0].clone()).unwrap();
    assert!(result.success, "stored client peer id to decider's kv");
}
*/

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

fn package_items_names(distro: &PackageDistro) -> Vec<String> {
    distro
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(distro.spells.iter().map(|s| s.name.clone()))
        .collect()
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

fn parse_logs_req(body: &[u8]) -> Option<LogsReq> {
    let mut req = serde_json::from_slice::<Value>(body).ok()?;
    let params = req.as_object()?.get("params")?;
    let mut logs = serde_json::from_value::<Vec<LogsReq>>(params.clone()).ok()?;
    let logs = logs.pop()?;
    Some(logs)
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

    fn empty_logs() -> Value {
        json!(
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": []
        })
    }

    fn log_test_app1(log: LogsReq) -> Value {
        let block_number1 = log.from_block;
        // Encoded CID (url-downloader): bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m
        // TODO: generate this on fly
        json!(
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": [
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x1",
                    "blockHash": "0x2",
                    "blockNumber": block_number1,
                    "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                    "data": "0x000000000000000000000000ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b9900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3",
                    "topics": [
                        "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                        log.topics[1]
                    ]
                },
            ]
        })
    }
}

/*
#[derive(Debug)]
struct ExecutionStep(Arc<Mutex<u32>>);

impl ExecutionStep {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(0)))
    }

    fn view(&self) -> ExecutionStepView {
        ExecutionStepView(self.0.clone())
    }
}

impl Drop for ExecutionStep {
    fn drop(&mut self) {
        if (self.0.lock().unwrap().clone()) != u32::MAX {
            panic!("ExecutionStep wasn't finished")
        }
    }
}

#[derive(Clone, Debug)]
struct ExecutionStepView(Arc<Mutex<u32>>);

impl ExecutionStepView {
    fn expect(&mut self, step: u32) {
        let mut lock = self.0.lock().unwrap();
        if step != lock.clone() {
            panic!("ExecutionStep expected {}, but got {}", step, lock);
        }
        *lock += 1;
    }

    fn finish(&mut self) {
        *self.0.lock().unwrap() = u32::MAX;
    }
}
*/

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct JrpcReq {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Vec<Value>,
}

struct ServerHandle {
    url: String,
    shutdown: Sender<()>,
    shutdown_confirm: Option<task::JoinHandle<()>>,
}

impl ServerHandle {
    async fn shutdown(mut self) {
        self.shutdown.try_send(()).unwrap();
        self.shutdown_confirm.take().unwrap().await.unwrap();
    }
}

/*
impl Drop for ServerHandle {
    fn drop(&mut self) {
        if !self.shutdown_confirm.is_none() {
            panic!("ServerHandle wasn't shutdown. Call shutdown() instead of dropping it");
        }
    }
}
 */

fn run_test_server<T, S>(handler: T) -> ServerHandle
where
    T: Fn(String, Vec<Value>) -> S + Send + Sync + Copy + 'static,
    S: Future<Output = Value> + Send + Sync,
{
    let process_http_request =
        async move |mut req: Request<Body>| -> Result<Response<Body>, Infallible> {
            let raw_body = req.body_mut();
            let mut buf = body::aggregate(raw_body).await.unwrap();
            let body = buf.copy_to_bytes(buf.remaining()).to_vec();
            let raw_request = serde_json::from_slice::<Value>(&body).unwrap();
            /*
            if raw_request.is_array() {
                // if it's a batch request
                let batch_req = serde_json::from_value::<Vec<JrpcReq>>(raw_request)?;
                let mut expect_id = 0;
                let mut result = Vec::new();
                for req in batch_req {
                    let result = Self::check_request(&req, expect_id);
                    if result.is_ok() {
                        result.push(handler(req.method, req.params));
                    }
                    expect_id += 1;
                }

                // do smth with result
            } else {

             */
            let req = serde_json::from_value::<JrpcReq>(raw_request).unwrap();
            assert_eq!(req.jsonrpc, "2.0", "wrong jsonrpc version: {}", req.jsonrpc);
            assert_eq!(req.id, 0, "wrong jsonrpc id: {}", req.id);
            let result = handler(req.method, req.params).await;
            let response_body: Vec<u8> = serde_json::to_string(&json!({
                    "jsonrpc": "2.0",
                    "id": 0,
                    "result": result,
            }))
            .unwrap()
            .into();
            Ok::<Response<Body>, Infallible>(Response::new(Body::from(response_body)))
        };
    let address = SocketAddr::from(([127, 0, 0, 1], 0));
    let make_service = make_service_fn(move |_conn| {
        let service = service_fn(move |req| process_http_request(req));
        async move { Ok::<_, Infallible>(service) }
    });
    let server = hyper::server::Server::bind(&address).serve(make_service);
    let address = server.local_addr();
    let (shutdown_send, mut shutdown_receive) = channel(1);
    let graceful = server.with_graceful_shutdown((async move || {
        shutdown_receive.recv().await.unwrap();
    })());
    let handle = task::spawn((async move || {
        graceful.await.unwrap();
    })());
    ServerHandle {
        url: format!("http://{}/", address),
        shutdown: shutdown_send,
        shutdown_confirm: Some(handle),
    }
}

#[tokio::test]
async fn test_deploy_new_deal_new_server_all_new_buy_now() {
    //let mut execution_step = ExecutionStep::new();
    //let mut execution_step_view = execution_step.view();

    enable_decider_logs();

    let mut server = run_test_server(async move |method, params| match method.as_str() {
        "eth_blockNumber" => {
            assert!(false, "eth_getLogs shouldn't be called");

            json!("0x10")
        }
        "eth_getLogs" => {
            let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
            // Encoded CID (url-downloader): bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m
            // TODO: generate this on the fly
            json!([
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x1",
                    "blockHash": "0x2",
                    "blockNumber": log.from_block,
                    "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                    "data": "0x000000000000000000000000ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b9900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3",
                    "topics": [
                        "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                        log.topics[1]
                    ]
                },
            ])
        }
        "eth_sendRawTransaction" => {
            json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
        }
        "eth_getTransactionCount" => json!("0x1"),
        "eth_gasPrice" => json!("0x3b9aca07"),
        _ => panic!("unexpected method: {}", method),
    });

    //execution_step_view.expect(0);
    let distro = make_distro_with_api(server.url.clone());
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

    println!("1");
    // how to wait until decider is over?
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    println!("1");

    let mut result = client
        .execute_particle(
            r#"
        (seq
            (seq
                (call relay ("decider" "get_u32") ["counter"] counter)
                (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
            )
            (call client ("return" "") [counter deals])
        )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string())
            },
        )
        .await
        .unwrap();
    println!("1");

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
    // execution_step_view.finish();

    // 2. Check that we can find worker_id by deal_id
    let result = client
        .execute_particle(
            r#"
            (seq
                (call relay ("worker" "get_worker_id") [deal_id] resolved_worker_id)
                (call client ("return" "") [resolved_worker_id])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
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

    let result = client
        .execute_particle(
            r#"
            (seq
                (seq
                    (call relay ("op" "noop") [])
                    (call worker_id ("srv" "list") [] list)
                )
                (call client ("return" "") [list])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
                "worker_id" => json!(deal.worker_id),
            },
        )
        .await
        .unwrap();

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
    let mut result = client
        .execute_particle(
            r#"
            (seq
                (seq
                    (call relay ("op" "noop") [])
                    (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
                )
                (call client ("return" "") [cid])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
                "worker_id" => json!(deal.worker_id),
            },
        )
        .await
        .unwrap();
    let result = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");

    server.shutdown().await
}

#[tokio::test]
async fn test_deploy_new_deal() {
    //let mut execution_step = ExecutionStep::new();
    //let mut execution_step_view = execution_step.view();

    enable_decider_logs();
    let mut server = mockito::Server::new();
    let url = server.url();
    let mock_block_number = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::Json(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_blockNumber",
            "params": [],
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "result": "0x10"
            }))
            .unwrap(),
        )
        .expect_at_least(1)
        .create();

    let mock_logs = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::PartialJson(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_getLogs"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(|req| {
            let log = parse_logs_req(req.body().expect("mock request body isn't read"))
                .expect("can't parse eth_getLogs request");
            serde_json::to_string(&TestApp::log_test_app1(log))
                .unwrap()
                .into()
        })
        .expect_at_least(1)
        .create();

    let mock_register_worker = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::PartialJson(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_sendRawTransaction"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(|req| {
            let body = req.body().expect("mock request body isn't read");
            let result = serde_json::from_slice::<Value>(body);
            println!("eth_sendRawTransaction: {:?}", result);

            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "result": "0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05"
            }))
            .unwrap()
            .into()
        })
        .expect_at_least(1)
        .create();

    let mock_nonce = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::PartialJson(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_getTransactionCount"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(|req| {
            let body = req.body().expect("mock request body isn't read");
            let result = serde_json::from_slice::<Value>(body);
            println!("eth_getTransactionCount: {:?}", result);
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "result":"0x1",
            }))
            .unwrap()
            .into()
        })
        .expect_at_least(1)
        .create();

    // let mut execution_step_view_2 = execution_step.view();
    let mock_nonce = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::PartialJson(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_gasPrice"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(move |req| {
            //    execution_step_view_2.clone().expect(1);
            let body = req.body().expect("mock request body isn't read");
            let result = serde_json::from_slice::<Value>(body);
            println!("eth_gasPrice: {:?}", result);
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "result":"0x3b9aca07",
            }))
            .unwrap()
            .into()
        })
        .expect_at_least(1)
        .create();

    let invalid_mock = server
        .mock("POST", "/")
        .expect(0)
        .with_status(404)
        .with_body_from_request(move |req| {
            println!(
                "Invalid: {:?}",
                String::from_utf8(req.body().unwrap().to_vec())
            );
            "invalid mock was hit. Check that request body matches 'match_body' clause'".into()
        })
        .create();

    //execution_step_view.expect(0);
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

    let mut result = client
        .execute_particle(
            r#"
        (seq
            (seq
                (call relay ("decider" "get_u32") ["counter"] counter)
                (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
            )
            (call client ("return" "") [counter deals])
        )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string())
            },
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
    // execution_step_view.finish();

    // 2. Check that we can find worker_id by deal_id
    let result = client
        .execute_particle(
            r#"
            (seq
                (call relay ("worker" "get_worker_id") [deal_id] resolved_worker_id)
                (call client ("return" "") [resolved_worker_id])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
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

    let result = client
        .execute_particle(
            r#"
            (seq
                (seq
                    (call relay ("op" "noop") [])
                    (call worker_id ("srv" "list") [] list)
                )
                (call client ("return" "") [list])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
                "worker_id" => json!(deal.worker_id),
            },
        )
        .await
        .unwrap();

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
    let mut result = client
        .execute_particle(
            r#"
            (seq
                (seq
                    (call relay ("op" "noop") [])
                    (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
                )
                (call client ("return" "") [cid])
            )
        "#,
            hashmap! {
                "relay" => json!(client.node.to_string()),
                "client" => json!(client.peer_id.to_string()),
                "worker_id" => json!(deal.worker_id),
            },
        )
        .await
        .unwrap();
    let result = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");
}

///
/// Test how Decider calculates the block to poll.
/// Block numbers sequence:
/// 1. 0x0  -- Decider should be able to poll from the beginning and don't break
///            In the eth_getLogs request: we can check that fromBlock is 0x0
///
/// 2. 0x10 -- the number is less then the range decider polls
///            Decider should move it's left boundary to this block
///            In the eth_getLogs request: we can check that fromBlock is 0x1
///
/// 3. 0x10 -- again the same number in not very realistic case when Decider is too fast and the chain is too slow
///            Decider shouldn't move it's left boundary anywhere
///            In the eth_getLogs request: we can check that fromBlock is 0x11
///
/// 4. 0xffffff -- big number which Decider shouldn't be able to process during one iteration
///            Decider should move to some boundary which isn't the previous one and isn't this big block number
///            In the eth_getLogs request: we can check that fromBlock is 0x11
///
#[tokio::test]
async fn test_left_boundary_idle() {
    enable_decider_logs();
    //log_utils::enable_logs();

    let block_numbers = Arc::new(vec!["0x0", "0x10", "0x10", "0xffffff"]);
    // Assuming that Decider check 500 blocks in one request
    let expected_last_seen = vec!["0x0", "0x10", "0x10", "0x205"];
    let expected_from_blocks = vec!["0x0", "0x1", "0x11", "0x11"];

    let from_blocks = Arc::new(Mutex::new(Vec::new()));
    let request_number = Arc::new(Mutex::new(0usize));
    let max_req_number = block_numbers.len();
    let block_numbers_local = block_numbers.clone();

    let mut server = mockito::Server::new();
    let url = server.url();
    let mock_block_number = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::Json(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_blockNumber",
            "params": [],
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(move |_req| {
            let mut n = request_number.lock().unwrap();
            let result = block_numbers[*n];
            if *n < max_req_number {
                *n += 1;
            }

            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "result": result,
            }))
            .unwrap()
            .into()
        })
        .expect_at_least(max_req_number)
        .create();

    let from_blocks_local = from_blocks.clone();
    let mock_logs = server
        .mock("POST", "/")
        .match_body(mockito::Matcher::PartialJson(json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "eth_getLogs"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_request(move |req| {
            let x = serde_json::from_slice::<JrpcReq>(req.body().unwrap()).unwrap();
            let from_block = x.params[0].as_object().unwrap()["fromBlock"]
                .as_str()
                .unwrap();
            from_blocks_local
                .clone()
                .lock()
                .unwrap()
                .push(from_block.to_string());
            serde_json::to_string(&TestApp::empty_logs())
                .unwrap()
                .into()
        })
        .expect_at_least(max_req_number)
        .create();

    let invalid_mock = server
        .mock("POST", "/")
        .expect(0)
        .with_status(404)
        .with_body_from_request(move |req| {
            println!(
                "Invalid: {:?}",
                String::from_utf8(req.body().unwrap().to_vec())
            );
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "id": 0,
                "error": json!({
                    "code": -32600,
                    "message": "Invalid Request"
                })
            }))
            .unwrap()
            .into()
        })
        .create();

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
    update_config(&mut client, &oneshot_config).await.unwrap();

    // In the loop we check that decider sees the same block number as we set in the mock
    // But on the last iteration we return very big block number with the decider shouldn't be able to
    // process during one iteration
    for step in 0..max_req_number {
        println!("step {step}");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let result = execute(
            &mut client,
            r#" (call relay ("decider" "get_string") ["last_seen_block"] last_seen) "#,
            "last_seen",
            hashmap! {},
        )
        .await
        .unwrap();

        let last_seen = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
        assert!(!last_seen.absent, "no last_seen_block in KV");
        assert_eq!(
            last_seen.str, expected_last_seen[step],
            "expected block number is different"
        );

        {
            let from_blocks = from_blocks.lock().unwrap();
            assert_eq!(
                from_blocks.len(),
                step + 1,
                "decider didn't make eth_getLogs req on step {}",
                step
            );
            assert_eq!(
                from_blocks[step], expected_from_blocks[step],
                "decider set wrong fromBlock"
            );
        }

        // on the last iteration
        if step < max_req_number - 1 {
            // run decider one more time
            let result = update_config(&mut client, &oneshot_config).await.unwrap();
            assert_eq!(result.as_slice(), &["done"], "can't update trigger config");
        }
    }
    /*
    // Here we check that the last_seen_block ISN'T equal to the large block number
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let result = execute(
        &mut client,
        r#" (call relay ("decider" "get_string") ["last_seen_block"] last_seen) "#,
        "last_seen",
        hashmap! {},
    )
    .await
    .unwrap();
    let last_seen = serde_json::from_value::<StringValue>(result[0].clone()).unwrap();
    assert!(!last_seen.absent, "no last_seen_block in KV");
    assert_ne!(
        last_seen.str,
        block_numbers_local[max_req_number - 1],
        "last_seen shouldn't change to the large block number"
    );
    assert_ne!(
        last_seen.str,
        block_numbers_local[max_req_number - 2],
        "last_seen shouldn't be equal to the previous block number"
    );

    {
        let from_blocks = from_blocks.lock().unwrap();
        assert_eq!(
            from_blocks.len(),
            max_req_number,
            "decider didn't make eth_getLogs req on step {}",
            step
        );
        assert_eq!(
            from_blocks[max_req_number - 1],
            expected_from_blocks[max_req_number - 1],
            "decider set wrong fromBlock"
        );
    }
    */

    mock_block_number.assert();
    mock_logs.assert();
    invalid_mock.assert();
}

pub mod test_rpc_server;

use connected_client::ConnectedClient;
use created_swarm::{make_swarms_with_cfg, CreatedSwarm};
use decider_distro::DeciderConfig;
use eyre::WrapErr;
use fluence_app_service::TomlMarineConfig;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::{ScriptValue, StringListValue, StringValue};
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};
use server_config::system_services_config::{AquaIpfsConfig, SystemServicesConfig};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use system_services::{PackageDistro, ServiceDistro, SpellDistro};

pub const DEAL_IDS: &[&'static str] = &[
    "ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b99",
    "880a53a54785df22ba804aee81ce8bd0d45bdedc",
    "67b2ad3866429282e16e55b715d12a77f85b7ce8",
    "1234563866429282e16e55b715d12a77f85b7cc9",
    "991b64a54785df22ba804aee81ce8bd0d45bdabb",
    "3665748409e712cd91b428c18e07a8e37b44c47e",
];

pub const IPFS_MULTIADDR: &str = "/ip4/127.0.0.1/tcp/5001";

pub fn setup_aqua_ipfs() -> AquaIpfsConfig {
    let mut config = AquaIpfsConfig::default();
    static IPFS_CLI_PATH: Option<&str> = option_env!("IPFS_CLI_PATH");
    if let Some(path) = IPFS_CLI_PATH {
        config.ipfs_binary_path = path.to_string();
    }
    config.external_api_multiaddr = IPFS_MULTIADDR.to_string();
    config.local_api_multiaddr = IPFS_MULTIADDR.to_string();
    config
}

pub fn setup_system_config() -> SystemServicesConfig {
    let mut config = SystemServicesConfig::default();
    config.aqua_ipfs = setup_aqua_ipfs();
    config
}

pub async fn setup_swarm(distro: PackageDistro) -> CreatedSwarm {
    let mut swarms = make_swarms_with_cfg(1, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        let config = setup_system_config();
        cfg.allowed_binaries = vec![
            config.aqua_ipfs.ipfs_binary_path.clone(),
            config.connector.curl_binary_path.clone(),
        ];
        cfg.override_system_services_config = Some(config);
        cfg
    })
    .await;
    swarms.remove(0)
}

pub async fn setup_nox(distro: PackageDistro) -> (CreatedSwarm, ConnectedClient) {
    let swarm = setup_swarm(distro).await;
    let client = ConnectedClient::connect_with_keypair(
        swarm.multiaddr.clone(),
        Some(swarm.management_keypair.clone()),
    )
    .await
    .unwrap();
    (swarm, client)
}

pub fn enable_decider_logs() {
    let namespaces = vec![
        "run-console=debug",
        "chain_connector=debug",
        /*
        "spell_event_bus=trace",
        "system_services=debug",
        "ipfs_effector=debug",
        "ipfs_pure=debug",
        "particle_reap=debug",
        "aquamarine::actor=debug",
        "aquamarine::aqua_runtime=off",
        "aquamarine=debug",
        */
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

pub fn package_items_names(distro: &PackageDistro) -> Vec<String> {
    distro
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(distro.spells.iter().map(|s| s.name.clone()))
        .collect()
}

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
        worker_ipfs_multiaddr: IPFS_MULTIADDR.to_string(),
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
        worker_ipfs_multiaddr: IPFS_MULTIADDR.to_string(),
        chain_api_endpoint: api,
        chain_network_id: 11,
        chain_contract_block_hex: "0x0".to_string(),
        chain_matcher_addr: "0x0".to_string(),
        chain_workers_gas: 210_00,
        chain_wallet_key: "0xfdc4ba94809c7930fe4676b7d845cbf8fa5c1beae8744d959530e5073004cf3f"
            .to_string(),
    };
    // decider will run once
    let trigger_config = TriggerConfig::default();
    make_distro(trigger_config, decider_settings)
}

pub fn make_distro_with_api_and_config(api: String, config: TriggerConfig) -> PackageDistro {
    let decider_settings = DeciderConfig {
        // worker will run once
        worker_period_sec: 0,
        worker_ipfs_multiaddr: IPFS_MULTIADDR.to_string(),
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

pub async fn execute(
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

pub async fn update_config(
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

// God left me here
pub fn modify_decider_spell_script(
    tmp_dir: PathBuf,
    decider_spell_id: String,
    updated_script: String,
) {
    let script_path: PathBuf = tmp_dir.join(
        [
            "services",
            "workdir",
            &decider_spell_id,
            "tmp",
            "script.air",
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    fs::write(&script_path, updated_script).unwrap();
}

pub async fn update_decider_script_for_tests(client: &mut ConnectedClient, test_dir: PathBuf) {
    let result = execute(
        client,
        r#"
            (seq
                (call relay ("srv" "resolve_alias_opt") ["decider"] id)
                (call relay ("decider" "get_script_source_from_file") [] script)
            )
    "#,
        "id script",
        hashmap! {},
    )
    .await
    .unwrap();
    assert_eq!(
        result[0].as_array().unwrap().len(),
        1,
        "can't resolve `decider` alias"
    );
    let decider_id = result[0].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .to_string();
    let script = serde_json::from_value::<ScriptValue>(result[1].clone()).unwrap();
    assert!(script.success, "can't get decider script");

    let updated_script = format!(
        r#"
        (seq
            {script}
            (call "{client}" ("return" "") ["done"])
        )
    "#,
        client = client.peer_id,
        script = script.source_code,
    );

    modify_decider_spell_script(test_dir, decider_id, updated_script);
}

pub fn oneshot_config() -> TriggerConfig {
    let mut config = TriggerConfig::default();
    config.clock.start_sec = 1;
    config
}

pub async fn wait_worker_spell_stopped(
    client: &mut ConnectedClient,
    worker_id: String,
    timeout_per_try: Duration,
) {
    for _ in 0..5 {
        // if only we can import these keys from Aqua files
        let result = execute(
            client,
            r#"
            (seq
                (call relay ("op" "noop") [])
                (call worker ("worker-spell" "list_get_strings") ["__installation_spell_status__"] status)
            )
        "#,
            r#"status"#,
            hashmap! {
                "worker" => json!(worker_id),
            },
        )
            .await
            .wrap_err("getting installation spell status")
            .unwrap();

        assert!(!result.is_empty(), "no result from the worker-spell");

        let strings = serde_json::from_value::<StringListValue>(result[0].clone()).unwrap();
        assert!(
            strings.success,
            "can't get installation spell status: {}",
            strings.error
        );

        if !strings.strings.is_empty() {
            #[derive(Deserialize, Debug)]
            struct State {
                state: String,
            }
            let last_status = strings.strings.last().unwrap();
            let state = serde_json::from_str::<State>(last_status).unwrap();
            let in_progress_statuses = ["INSTALLATION_IN_PROGRESS", "NOT_STARTED"];
            println!("WORKER STATUS: {}", state.state);
            if !in_progress_statuses.contains(&state.state.as_str()) {
                assert_eq!(
                    state.state, "INSTALLATION_SUCCESSFUL",
                    "wrong installation spell status"
                );
                break;
            }
        }
        tokio::time::sleep(timeout_per_try).await;
    }
}

pub async fn wait_decider_stopped(client: &mut ConnectedClient) {
    let decider_stopped = client
        .receive_args()
        .await
        .wrap_err("wait decider")
        .unwrap();

    assert_eq!(
        decider_stopped[0].as_str().unwrap(),
        "done",
        "decider ended with a different return status"
    );
}

pub async fn get_worker_app_cid(client: &mut ConnectedClient, worker_id: &String) -> String {
    let mut result = execute(
        client,
        r#"
        (seq
            (call relay ("op" "noop") [])
            (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
        )
        "#,
        "cid",
        hashmap! {
            "worker_id" => json!(worker_id),
        },
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");
    serde_json::from_str::<String>(&result.str).unwrap()
}

#[derive(Deserialize, Debug)]
pub struct DealState {
    pub left_boundary: String,
}

pub async fn get_deal_state(client: &mut ConnectedClient, deal_id: &String) -> DealState {
    let mut result = execute(
        client,
        r#"
            (call relay ("decider" "get_string") [deal_id] deal_state)
        "#,
        "deal_state",
        hashmap! {
            "deal_id" => json!(deal_id)
        },
    )
    .await
    .unwrap();
    let str = serde_json::from_value::<StringValue>(result.remove(0))
        .wrap_err("get deal_state")
        .unwrap();
    assert!(!str.absent, "no state for deal {}", deal_id);
    assert!(
        str.success,
        "can't get state for deal {}: {}",
        deal_id, str.error
    );
    serde_json::from_str::<DealState>(&str.str)
        .wrap_err("parse deal_state")
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct JoinedDeal {
    pub deal_id: String,
    pub worker_id: String,
}

pub fn parse_joined_deals(deals: Value) -> Vec<JoinedDeal> {
    let deals = serde_json::from_value::<StringListValue>(deals).unwrap();
    assert!(deals.success);
    deals
        .strings
        .iter()
        .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
        .collect()
}

pub async fn get_joined_deals(mut client: &mut ConnectedClient) -> Vec<JoinedDeal> {
    let mut deals = execute(
        &mut client,
        r#"
            (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
        "#,
        "deals",
        hashmap! {},
    )
    .await
    .unwrap();
    parse_joined_deals(deals.remove(0))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogsReq {
    pub address: String,
    #[serde(deserialize_with = "hex_u32_deserialize")]
    pub from_block: u32,
    #[serde(deserialize_with = "hex_u32_deserialize")]
    pub to_block: u32,
    pub topics: Vec<String>,
}

pub fn hex_u32_deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    if s.starts_with("0x") {
        u32::from_str_radix(&s[2..], 16).map_err(serde::de::Error::custom)
    } else {
        Err(serde::de::Error::custom(format!(
            "Invalid hex format: {}",
            s
        )))
    }
}

pub fn to_hex(x: u32) -> String {
    format!("0x{:x}", x)
}

pub struct TestApp {
    pub cid: String,
    pub services_names: Vec<String>,
}

impl TestApp {
    // Predefined url_downloader app
    pub fn test_app1() -> Self {
        Self {
            cid: "bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m".to_string(),
            services_names: vec!["url_downloader".to_string()],
        }
    }

    pub fn log_test_app1(deal_id: &str, block: u32, host_topic: &str) -> Value {
        // Encoded CID (url-downloader): bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m
        // TODO: generate this on fly
        json!(
            {
                "removed": false,
                "logIndex": "0xb",
                "transactionIndex": "0x0",
                "transactionHash": "0x1",
                "blockHash": "0x2",
                "blockNumber": to_hex(block),
                "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                "data": format!("0x000000000000000000000000{deal_id}00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3"),
                "topics": [
                    "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                    host_topic
                ]
            }
        )
    }

    pub fn test_app2() -> Self {
        Self {
            cid: "bafkreicdwo6xrumiqc5a7oghbkay4tmmejlmokpweyut5uhe2tehsycvmu".to_string(),
            services_names: vec!["newService1".to_string()],
        }
    }

    pub fn log_test_app2(deal_id: &str, block: u32, host_topic: &str) -> Value {
        // CID: bafkreicdwo6xrumiqc5a7oghbkay4tmmejlmokpweyut5uhe2tehsycvmu
        // some default fcli app name: newService1
        json!(
            {
                "removed": false,
                "logIndex": "0x5",
                "transactionIndex": "0x0",
                "transactionHash": "0x54ae26abd742239bb492abe1b9ee98c27edde8454d7acc2e398ad365914071b5",
                "blockHash": "0x4e301dc22b7eb4bfd9c22865d36dfb68d4eb96a218f7b5f92c71760497e111ca",
                "blockNumber": to_hex(block),
                "address": "0x0f68c702dc151d07038fa40ab3ed1f9b8bac2981",
                "data": format!("0x000000000000000000000000{deal_id}88924347d3eddcdaa6e6a3844bea08cfc8dae2d5b43d8c6fa35de5fd9ab6cc750000000000000000000000000000000000000000000000000000000000000103015512200000000000000000000000000000000000000000000000000000000043b3bd78d18880ba0fb8c70a818e4d8c2256c729f626293ed0e4d4c879605565"),
                "topics": [
                  "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                  host_topic
                ]
            }
        )
    }
}

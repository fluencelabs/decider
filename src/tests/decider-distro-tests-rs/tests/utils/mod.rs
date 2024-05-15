pub mod control;
pub mod default;
pub mod distro;
pub mod spell;
pub mod state;

pub mod chain;
pub mod setup;
mod test_apps;
pub mod test_rpc_server;

pub use test_apps::TestApp;

use connected_client::ConnectedClient;
use created_swarm::fluence_spell_dtos::trigger_config::TriggerConfig;
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn enable_decider_logs() {
    let namespaces = vec![
        "run-console=debug",
        "spell=debug",
        "ipfs_effector=debug",
        "ipfs_pure=debug",
        "spell_event_bus=trace",
        "system_services=debug",
        "particle_reap=debug",
        "aquamarine::actor=debug",
        "aquamarine::aqua_runtime=off",
        "aquamarine=debug",
        "nox=debug",
        "chain_listener=debug",
        "chain-connector=debug",
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

pub fn oneshot_config() -> TriggerConfig {
    let mut config = TriggerConfig::default();
    config.clock.start_sec = 1;
    config
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

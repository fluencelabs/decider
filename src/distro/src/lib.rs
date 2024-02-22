use std::collections::HashMap;

use fluence_spell_dtos::trigger_config::TriggerConfig;
use maplit::hashmap;
use serde_json::{json, Value as JValue};

pub use build_info::PKG_VERSION as VERSION;

const CONNECTOR: &'static [u8] = include_bytes!("../decider-spell/chain_connector.wasm");
const CURL_ADAPTER: &'static [u8] = include_bytes!("../decider-spell/curl_adapter.wasm");
const CONFIG: &'static [u8] = include_bytes!("../decider-spell/Config.toml");

const DECIDER_SPELL: &'static str = include_str!("../decider-spell/poll.main.air");
const WORKER_SPELL: &'static str = include_str!("../decider-spell/deal_spell.main.air");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub struct DistrService {
    pub name: &'static str,
    pub config: &'static [u8],
    pub modules: HashMap<&'static str, &'static [u8]>,
}

pub fn connector_service_modules() -> DistrService {
    DistrService {
        // The name is used by the decider, so we don't need to explicitly pass the service id of the connector service
        // The name is taken from the config. Would be nice one day to automatically take it from the project itself.
        name: "chain_connector",
        config: CONFIG,
        modules: hashmap! {
            "chain_connector" => CONNECTOR,
            "curl_adapter" => CURL_ADAPTER,
        },
    }
}

pub struct DistrSpell {
    /// AIR script of the spell
    pub air: &'static str,
    /// Initial key-value records for spells KV storage
    pub kv: HashMap<&'static str, JValue>,
}

/// Decider's configuration needed for the correct decider start-up
#[derive(Debug)]
pub struct DeciderConfig {
    /// Multiaddr of the IPFS node from which to take worker definitions
    pub worker_ipfs_multiaddr: String,
    /// How often to run the worker-spell for updates/healthchecks
    pub worker_period_sec: u32,
    /// URI of the RPC for decider to communicate with the chain (poll, send tx)
    pub chain_api_endpoint: String,
    /// The block number from which to poll new deals in hex format
    pub chain_contract_block_hex: String,
    /// Address of the Market contract
    pub chain_market_addr: String,
}

pub fn decider_spell(config: DeciderConfig) -> DistrSpell {
    let mut worker_config = TriggerConfig::default();
    worker_config.clock.start_sec = 1;
    worker_config.clock.period_sec = config.worker_period_sec;

    DistrSpell {
        air: DECIDER_SPELL,
        kv: hashmap! {
            "worker_settings" => json!({
                "script": WORKER_SPELL,
                "config": worker_config,
                "ipfs": config.worker_ipfs_multiaddr
            }),
            "from_block" => json!(config.chain_contract_block_hex),
            "chain" => json!( {
                "api_endpoint": config.chain_api_endpoint,
                "market": config.chain_market_addr,
            }),
        },
    }
}

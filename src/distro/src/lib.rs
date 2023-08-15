use std::collections::HashMap;

use fluence_spell_dtos::trigger_config::TriggerConfig;
use maplit::hashmap;
use serde_json::{json, Value as JValue};

pub use build_info::PKG_VERSION as VERSION;

const CONNECTOR: &'static [u8] = include_bytes!("../decider-spell/fluence_aurora_connector.wasm");
const CURL_ADAPTER: &'static [u8] = include_bytes!("../decider-spell/curl_adapter.wasm");
const CONFIG: &'static [u8] = include_bytes!("../decider-spell/Config.toml");

const DECIDER_SPELL: &'static str = include_str!("../decider-spell/poll/poll.main.air");
const WORKER_SPELL: &'static str = include_str!("../decider-spell/deal_spell/deal_spell.main.air");

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
        name: "fluence_aurora_connector",
        config: CONFIG,
        modules: hashmap! {
            "fluence_aurora_connector" => CONNECTOR,
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
    /// The network of the chain from which the decider polls deals
    pub chain_network: String,
    /// The block number from which to poll new deals in hex format
    pub chain_contract_block_hex: String,
    /// Address of the Matcher contract
    pub chain_matcher_addr: String,
    /// How much gas is needed to register a worker
    pub chain_workers_gas: u64,
    /// Private key to send `setWorker` tx to chain to register a worker
    pub chain_wallet_key: String,
}

pub fn decider_spell(config: DeciderConfig) -> DistrSpell {
    let mut worker_config = TriggerConfig::default();
    worker_config.clock.start_sec = 1;
    worker_config.clock.period_sec = config.worker_period_sec;

    DistrSpell {
        air: DECIDER_SPELL,
        kv: hashmap! {
            "worker_script" => json!(WORKER_SPELL),
            "worker_config" => json!(worker_config),
            "worker_ipfs" => json!(config.worker_ipfs_multiaddr),
            "from_block" => json!(config.chain_contract_block_hex),
            "chain" => json!( {
                "api_endpoint": config.chain_network,
                "matcher": config.chain_matcher_addr, // "0x0f68c702dC151D07038fA40ab3Ed1f9b8BAC2981",
                "workers_gas": config.chain_workers_gas, // 210000,
                "wallet_key": config.chain_wallet_key, // "0xbb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a",
            }),
        },
    }
}

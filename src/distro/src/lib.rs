use std::collections::HashMap;
use maplit::hashmap;

pub const CURL_ADAPTER: &'static [u8] = include_bytes!("../../../target/wasm32-wasi/release/fluence_aurora_connector.wasm");
pub const CONNECTOR: &'static [u8] = include_bytes!("../../../target/wasm32-wasi/release/curl_adapter.wasm");
pub const CONFIG: &'static [u8] = include_bytes!("../../../.fluence/tmp/Config.toml");

pub const DECIDER_SPELL: &'static str = include_str!("../decider-spell/decider.main.air");
pub const WORKER_SPELL: &'static str = include_str!("../decider-spell/worker.main.air");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub use build_info::PKG_VERSION as VERSION;

pub fn modules() -> HashMap<&'static str, &'static [u8]> {
    hashmap! {
        "connector" => CONNECTOR,
        "curl_adapter" => CURL_ADAPTER,
    }
}

pub struct DistrSpell {
    /// AIR script of the spell
    pub air: &'static str,
    /// Initial key-value records for spells KV storage
    pub kv: HashMap<&'static str, &'static str>,
}

pub fn spells() -> HashMap<&'static str, DistrSpell> {
    hashmap! {
        "decider" => DistrSpell {
            air: DECIDER_SPELL,
            kv: hashmap! {
                "worker_script" => WORKER_SPELL,
            }
        }
    }
}

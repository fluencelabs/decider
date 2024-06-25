/*
 * Copyright 2024 Fluence DAO
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;

use fluence_spell_dtos::trigger_config::TriggerConfig;
use maplit::hashmap;
use serde_json::{json, Value as JValue};

pub use build_info::PKG_VERSION as VERSION;

const DECIDER_SPELL: &'static str = include_str!("../decider-spell/main.main.air");
const WORKER_SPELL: &'static str = include_str!("../decider-spell/worker_spell.main.air");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub struct DistrService {
    pub name: &'static str,
    pub config: &'static [u8],
    pub modules: HashMap<&'static str, &'static [u8]>,
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
        },
    }
}

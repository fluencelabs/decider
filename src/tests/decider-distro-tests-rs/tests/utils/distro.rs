use created_swarm::fluence_spell_dtos::trigger_config::TriggerConfig;
use created_swarm::system_services::{PackageDistro, SpellDistro};
use decider_distro::DeciderConfig;

use crate::utils::default::IPFS_MULTIADDR;

pub fn package_items_names(distro: &PackageDistro) -> Vec<String> {
    distro
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(distro.spells.iter().map(|s| s.name.clone()))
        .collect()
}

pub fn make_distro(trigger_config: TriggerConfig, settings: DeciderConfig) -> PackageDistro {
    let distro_spell = decider_distro::decider_spell(settings);
    let spell = SpellDistro {
        name: "decider".to_string(),
        air: distro_spell.air,
        kv: distro_spell.kv.clone(),
        trigger_config,
    };

    PackageDistro {
        name: "decider".to_string(),
        version: decider_distro::VERSION,
        services: vec![],
        spells: vec![spell],
        init: None,
    }
}

// Note that by default in these tests:
// - Decider is stopped and should run manually
// - Worker Spell is oneshot
pub fn make_distro_default() -> PackageDistro {
    let decider_settings = DeciderConfig {
        worker_period_sec: 0,
        worker_ipfs_multiaddr: IPFS_MULTIADDR.to_string(),
    };
    // let's try to run a decider cycle on demand by updating the config
    let trigger_config = TriggerConfig::default();
    make_distro(trigger_config, decider_settings)
}

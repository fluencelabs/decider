use crate::utils::default::IPFS_MULTIADDR;
use connected_client::ConnectedClient;
use created_swarm::system_services_config::{AquaIpfsConfig, SystemServicesConfig};
use created_swarm::{make_swarms_with_cfg, CreatedSwarm};
use system_services::PackageDistro;

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

/*
 * Nox Fluence Peer
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::str::FromStr;
use std::sync::Arc;

use clarity::PrivateKey;
use maplit::hashmap;
use tempfile::TempDir;

use connected_client::ConnectedClient;
use created_swarm::fluence_keypair::KeyPair;
use created_swarm::system_services::PackageDistro;
use created_swarm::system_services_config::{AquaIpfsConfig, SystemServicesConfig};
use created_swarm::{make_swarms_with_cfg, ChainConfig, CreatedSwarm};

use crate::utils::control::update_decider_script_for_tests;
use crate::utils::default::{IPFS_MULTIADDR, NETWORK_ID, WALLET_KEY};
use crate::utils::distro::make_distro_default;

pub fn setup_aqua_ipfs() -> AquaIpfsConfig {
    let mut config = AquaIpfsConfig::default();
    static IPFS_CLI_PATH: Option<&str> = option_env!("IPFS_CLI_PATH");
    if let Some(path) = IPFS_CLI_PATH {
        config.ipfs_binary_path = path.to_string().into();
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

pub async fn setup_swarm(
    distro: PackageDistro,
    peers: usize,
    url: String,
    tmp_dir: Option<Arc<TempDir>>,
    kp: Option<KeyPair>,
) -> Vec<CreatedSwarm> {
    let swarms = make_swarms_with_cfg(peers, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        let mut config = setup_system_config();
        cfg.allowed_binaries = vec![
            config.aqua_ipfs.ipfs_binary_path.clone(),
            config.connector.curl_binary_path.clone(),
        ];
        cfg.allowed_effectors = hashmap! {
            // curl-adepter effector; ./resources/upload.sh to see the effector CID
            "bafkreihbz4szbmix7tphbiprnbhnmjqavpzyndtntzrcbbcdnancd2cjae".to_string() => hashmap! {
                "curl".to_string() => config.connector.curl_binary_path.clone()
            }
        };
        // to make worker spell oneshot
        config.decider.worker_period_sec = 0;
        cfg.override_system_services_config = Some(config);

        cfg.chain_config = Some(ChainConfig {
            http_endpoint: url.clone(),
            core_contract_address: "core_contract".to_string(),
            cc_contract_address: "cc_contract".to_string(),
            market_contract_address: "market_contract".to_string(),
            network_id: NETWORK_ID,
            wallet_key: PrivateKey::from_str(WALLET_KEY).unwrap(),
            default_base_fee: None,
            default_priority_fee: None,
        });

        if let Some(tmp_dir) = &tmp_dir {
            cfg.tmp_dir = tmp_dir.clone();
        }

        if let Some(kp) = &kp {
            cfg.keypair = kp.clone();
        }

        cfg
    })
    .await;
    swarms
}

pub async fn setup_nox(url: String) -> (CreatedSwarm, ConnectedClient) {
    setup_nox_gen(make_distro_default(), url, None, None).await
}

pub async fn setup_nox_with(
    url: String,
    temp_dir: Arc<TempDir>,
    kp: KeyPair,
) -> (CreatedSwarm, ConnectedClient) {
    setup_nox_gen(make_distro_default(), url, Some(temp_dir), Some(kp)).await
}

async fn setup_nox_gen(
    distro: PackageDistro,
    url: String,
    temp_dir: Option<Arc<TempDir>>,
    kp: Option<KeyPair>,
) -> (CreatedSwarm, ConnectedClient) {
    let mut swarms = setup_swarm(distro, 1, url, temp_dir, kp).await;
    let swarm = swarms.remove(0);
    let client = setup_client(&swarm).await;
    (swarm, client)
}

async fn setup_client(swarm: &CreatedSwarm) -> ConnectedClient {
    let mut client = ConnectedClient::connect_with_keypair(
        swarm.multiaddr.clone(),
        Some(swarm.management_keypair.clone()),
    )
    .await
    .unwrap();
    update_decider_script_for_tests(
        &mut client,
        swarm.config.dir_config.persistent_base_dir.clone(),
    )
    .await;
    client
}

pub fn stop_nox(swarm: CreatedSwarm) -> Result<(), ()> {
    swarm.exit_outlet.send(())?;
    Ok(())
}

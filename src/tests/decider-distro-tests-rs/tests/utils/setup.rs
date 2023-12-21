use crate::utils::chain::LogsReq;
use crate::utils::default::{default_receipt, default_status, DEAL_STATUS_ACTIVE, IPFS_MULTIADDR};
use crate::utils::test_rpc_server::ServerHandle;
use crate::utils::*;
use connected_client::ConnectedClient;
use created_swarm::system_services_config::{AquaIpfsConfig, SystemServicesConfig};
use created_swarm::{make_swarms_with_cfg, CreatedSwarm};
use serde_json::json;
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

pub async fn setup_swarm(distro: PackageDistro, peers: usize) -> Vec<CreatedSwarm> {
    let swarms = make_swarms_with_cfg(peers, move |mut cfg| {
        cfg.enabled_system_services = vec!["aqua-ipfs".to_string()];
        cfg.extend_system_services = vec![distro.clone()];
        let mut config = setup_system_config();
        cfg.allowed_binaries = vec![
            config.aqua_ipfs.ipfs_binary_path.clone(),
            config.connector.curl_binary_path.clone(),
        ];
        // to make worker spell oneshot
        config.decider.worker_period_sec = 0;
        cfg.override_system_services_config = Some(config);
        cfg
    })
    .await;
    swarms
}

pub async fn setup_nox(distro: PackageDistro) -> (CreatedSwarm, ConnectedClient) {
    let mut swarms = setup_swarm(distro, 1).await;
    let swarm = swarms.remove(0);
    let client = ConnectedClient::connect_with_keypair(
        swarm.multiaddr.clone(),
        Some(swarm.management_keypair.clone()),
    )
    .await
    .unwrap();
    (swarm, client)
}

// Deploy the first deal for a peer as a part of the test setup process.
// The sequence of RPC calls describes **only** the flow when there are no other deals were previously joined.
pub async fn setup_rpc_deploy_deal(
    server: &mut ServerHandle,
    latest_block: u32,
    deal_id: &str,
    block_number: u32,
) -> Option<()> {
    setup_rpc_deploy_deals(server, latest_block, vec![(deal_id, block_number)]).await
}

pub async fn setup_rpc_deploy_deals(
    server: &mut ServerHandle,
    latest_block: u32,
    deals: Vec<(&str, u32)>,
) -> Option<()> {
    let expected_reqs = 2 + 5 * deals.len();
    for _ in 0..expected_reqs {
        let (method, params) = server.receive_request().await?;
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(latest_block)),
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                let logs = deals
                    .iter()
                    .map(|(deal_id, block_number)| {
                        TestApp::log_test_app1(deal_id, *block_number, log.topics[1].as_str())
                    })
                    .collect::<Vec<_>>();
                json!(logs)
            }
            "eth_sendRawTransaction" => {
                json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
            }
            "eth_getTransactionCount" => json!("0x1"),
            "eth_gasPrice" => json!("0x3b9aca07"),
            "eth_getTransactionReceipt" => default_receipt(),
            "eth_call" => default_status(),
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(response));
    }
    Some(())
}

// Expected RPC calls when nothing is happening on-chain, and
// the joined deal **don't** require transaction status updates.
pub async fn setup_rpc_empty_run(
    server: &mut ServerHandle,
    latest_block: u32,
    joined: usize,
) -> Option<()> {
    setup_rpc_empty_run_with_status(server, latest_block, DEAL_STATUS_ACTIVE, joined).await
}

pub async fn setup_rpc_empty_run_with_status(
    server: &mut ServerHandle,
    latest_block: u32,
    status: &str,
    joined: usize,
) -> Option<()> {
    let expected_reqs = 2 + 3 * joined;
    for _ in 0..expected_reqs {
        let (method, _params) = server.receive_request().await?;
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(latest_block)),
            "eth_getLogs" => {
                json!([])
            }
            "eth_call" => json!(status),
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(response));
    }
    Some(())
}

/*

Expected RPC calls:

eth_blockNumber
eth_getLogs (for new deals) -> new_deals
    for new_deal in new_deals {
        eth_gasPrice
        eth_getTransactionCount
        eth_sendRawTransaction
        eth_getTransactionReceipt
        eth_call (getStatus) <--- This call is batched, done after all logs are processed
    }

for joined_deal in joined_deals {
    eth_getLogs (for update)  <-- This request is batched
    eth_call (getStatus)      <-- This request is batched
    eth_getLogs (for removes) <-- This request is batched
    if joined_deal.tx_status == "pending":
      eth_getTransactionReceipt <-- This request is also batched
}

Batched requests in this test system are encoded as separate requests.

 */

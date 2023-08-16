use marine_rs_sdk::marine;

#[marine]
#[derive(Default)]
pub struct ChainInfo {
    // URL of a chain RPC
    pub api_endpoint: String,
    // Address of the matcher contract
    pub matcher: String,
    // How much gas is needed to register a worker
    pub workers_gas: u64,
    // Private key of the wallet
    pub wallet_key: String,
    // ID of the chain behind RPC. EIP-155: protection from replay on other chains.
    pub network_id: u64,
}

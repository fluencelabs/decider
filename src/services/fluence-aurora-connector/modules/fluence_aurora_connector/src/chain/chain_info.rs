use marine_rs_sdk::marine;

#[marine]
pub struct ChainInfo {
    // URL of a chain RPC
    pub api_endpoint: String,
    // Chain id of the chain behind RPC
    pub network_id: u64,
    // Address of the deal factory contract
    pub deal_factory: String,
    // Address of the matcher contract
    pub matcher: String,
    // Address of the worker contract
    pub workers: String,
    // How much gas is needed to register a worker
    pub workers_gas: u128,
    // private key of the wallet
    pub wallet_key: String,
}

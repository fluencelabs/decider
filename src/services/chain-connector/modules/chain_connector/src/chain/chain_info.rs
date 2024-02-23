use marine_rs_sdk::marine;

#[marine]
#[derive(Default)]
pub struct ChainInfo {
    // URL of a chain RPC
    pub api_endpoint: String,
    // Address of the market contract
    pub market: String,
}

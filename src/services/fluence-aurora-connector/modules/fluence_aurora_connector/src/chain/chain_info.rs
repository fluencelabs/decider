use marine_rs_sdk::marine;

// -- Data we need to poll new deals from aurora
// data ChainInfo:
//   -- Refers to which api endpoint to poll
//   api_endpoint: URL
//   -- Address of the deal factory contract
//   deal_factory: Address
//   -- Matcher contract address
//   matcher: Address
//   -- Provider wallet address
//   provider: Address

#[marine]
pub struct ChainInfo {
    pub api_endpoint: String,
    pub deal_factory: String,
    pub matcher: String,
    pub provider: String,
}

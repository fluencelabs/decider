use serde_json::{json, Value};

pub const DEFAULT_POLL_WINDOW_BLOCK_SIZE: u32 = 2000;

pub const DEAL_IDS: &[&'static str] = &[
    "ffa0611a099ab68ad7c3c67b4ca5bbbee7a58b99",
    "880a53a54785df22ba804aee81ce8bd0d45bdedc",
    "67b2ad3866429282e16e55b715d12a77f85b7ce8",
    "1234563866429282e16e55b715d12a77f85b7cc9",
    "991b64a54785df22ba804aee81ce8bd0d45bdabb",
    "3665748409e712cd91b428c18e07a8e37b44c47e",
];

pub const IPFS_MULTIADDR: &str = "/ip4/127.0.0.1/tcp/5001";

/// Wallet Private Key to pass to Nox in tests
pub const WALLET_KEY: &str = "0xfdc4ba94809c7930fe4676b7d845cbf8fa5c1beae8744d959530e5073004cf3f";

pub const NETWORK_ID: u64 = 11;

pub fn default_receipt() -> Value {
    json!({"status" : "0x1", "blockNumber": "0x300"})
}

pub const DEAL_STATUS_ACTIVE: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const DEAL_STATUS_NOT_ENOUGH_WORKERS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000003";
pub const DEAL_STATUS_ENDED: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000002";
pub fn default_status() -> Value {
    json!(DEAL_STATUS_ACTIVE)
}

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

pub fn default_receipt() -> Value {
    json!({"status" : "0x1", "blockNumber": "0x300"})
}

pub const DEAL_STATUS_ACTIVE: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const DEAL_STATUS_INACTIVE: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
pub const DEAL_STATUS_ENDED: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000002";
pub fn default_status() -> Value {
    json!(DEAL_STATUS_ACTIVE)
}

use crate::utils::to_hex;
use serde_json::{json, Value};

pub struct TestApp {
    pub cid: String,
    pub services_names: Vec<String>,
}

impl TestApp {
    // Predefined url_downloader app
    pub fn test_app1() -> Self {
        Self {
            cid: "bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m".to_string(),
            services_names: vec!["url_downloader".to_string()],
        }
    }

    pub fn log_test_app1(deal_id: &str, block: u32, host_topic: &str) -> Value {
        // Encoded CID (url-downloader): bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m
        // TODO: generate this on fly
        json!(
            {
                "removed": false,
                "logIndex": "0xb",
                "transactionIndex": "0x0",
                "transactionHash": "0x1",
                "blockHash": "0x2",
                "blockNumber": to_hex(block),
                "address": "0xb971228a3af887c8c50e7ab946df9def0d12cab2",
                "data": format!("0x000000000000000000000000{deal_id}00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000500155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e300000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3"),
                "topics": [
                    "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                    host_topic
                ]
            }
        )
    }

    pub fn test_app2() -> Self {
        Self {
            cid: "bafkreicdwo6xrumiqc5a7oghbkay4tmmejlmokpweyut5uhe2tehsycvmu".to_string(),
            services_names: vec!["newService1".to_string()],
        }
    }

    pub fn log_test_app2(deal_id: &str, block: u32, host_topic: &str) -> Value {
        // CID: bafkreicdwo6xrumiqc5a7oghbkay4tmmejlmokpweyut5uhe2tehsycvmu
        // some default fcli app name: newService1
        json!(
            {
                "removed": false,
                "logIndex": "0x5",
                "transactionIndex": "0x0",
                "transactionHash": "0x54ae26abd742239bb492abe1b9ee98c27edde8454d7acc2e398ad365914071b5",
                "blockHash": "0x4e301dc22b7eb4bfd9c22865d36dfb68d4eb96a218f7b5f92c71760497e111ca",
                "blockNumber": to_hex(block),
                "address": "0x0f68c702dc151d07038fa40ab3ed1f9b8bac2981",
                "data": format!("0x000000000000000000000000{deal_id}88924347d3eddcdaa6e6a3844bea08cfc8dae2d5b43d8c6fa35de5fd9ab6cc750000000000000000000000000000000000000000000000000000000000000103015512200000000000000000000000000000000000000000000000000000000043b3bd78d18880ba0fb8c70a818e4d8c2256c729f626293ed0e4d4c879605565"),
                "topics": [
                  "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                  host_topic
                ]
            }
        )
    }
}

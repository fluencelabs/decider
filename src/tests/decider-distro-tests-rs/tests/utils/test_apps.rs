use crate::utils::to_hex;
use cid::Cid;
use serde_json::{json, Value};
use std::str::FromStr;

pub struct TestApp {
    pub cid: String,
    pub services_names: Vec<String>,
}

impl TestApp {
    // Predefined url_downloader app
    pub fn test_app1() -> Self {
        Self {
            cid: "bafkreiekvwp2w7t7vw4jzjq4s4n4wc323c6dnexmy4axh6c7tiza5wxzm4".to_string(),
            services_names: vec!["url_downloader".to_string()],
        }
    }

    fn encoded_cid(&self) -> String {
        let x = Cid::from_str(&self.cid).unwrap();
        let bts = x.to_bytes();
        let prefix = hex::encode(bts[0..4].to_vec());
        let hash = hex::encode(bts[4..].to_vec());
        format!("{prefix}00000000000000000000000000000000000000000000000000000000{hash}")
    }

    pub fn log_test_app1(deal_id: &str, block: u32, host_topic: &str) -> Value {
        let test_app = Self::test_app1();
        let app_cid = test_app.encoded_cid();
        // Encoded CID (url-downloader)
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
                "data": format!("0x000000000000000000000000{deal_id}00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000050{app_cid}00000000000000000000000000000000000000000000000000000000000000036c9d5e8bcc73a422dd6f968f13cd6fc92ccd5609b455cf2c7978cbc694297853fef3b95696986bf289166835e05f723f0fdea97d2bc5fea0ebbbf87b6a866cfa5a5a0f4fa4d41a4f976e799895cce944d5080041dba7d528d30e81c67973bac3"),
                "topics": [
                    "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                    host_topic
                ]
            }
        )
    }

    pub fn log_test_app1_update(deal_id: &str) -> Value {
        let test_app = Self::test_app1();
        let app_cid = test_app.encoded_cid();
        json!(
              {
                "address": deal_id,
                "topics": [
                  "0x0e85c04920a2349be7d0f03a765fa172e5dabc0a4a9fc47acb81c07ce8d260d0",
                ],
                "data": format!("0x{app_cid}"),

                "blockNumber": "0x300",
                "transactionHash": "0xb825edf7da59840ce838a9ed70aa0aa6c54c322ca5d6f0be4f070766e46ebbd8",
                "transactionIndex": "0xb",
                "blockHash": "0x34ba65babca6f1ef44da5f75c7bb4335c7b7484178a74003de5df139ac6551ed",
                "logIndex": "0x26",
                "removed": false
              }
        )
    }

    pub fn test_app2() -> Self {
        Self {
            cid: "bafkreieodklcdlpj5skykwctr466425y3f2jz7jukzj3dujlvk3ptwffim".to_string(),
            services_names: vec!["newService1".to_string()],
        }
    }

    pub fn log_test_app2(deal_id: &str, block: u32, host_topic: &str) -> Value {
        let test_app = Self::test_app2();
        let app_cid = test_app.encoded_cid();
        json!(
            {
                "removed": false,
                "logIndex": "0x5",
                "transactionIndex": "0x0",
                "transactionHash": "0x54ae26abd742239bb492abe1b9ee98c27edde8454d7acc2e398ad365914071b5",
                "blockHash": "0x4e301dc22b7eb4bfd9c22865d36dfb68d4eb96a218f7b5f92c71760497e111ca",
                "blockNumber": to_hex(block),
                "address": "0x0f68c702dc151d07038fa40ab3ed1f9b8bac2981",
                "data": format!("0x000000000000000000000000{deal_id}88924347d3eddcdaa6e6a3844bea08cfc8dae2d5b43d8c6fa35de5fd9ab6cc750000000000000000000000000000000000000000000000000000000000000103{app_cid}"),
                "topics": [
                  "0x1c13422d2375fe8a96ddbe3f6e2efc794f2befbfe247217479ef4b68030d42c3",
                  host_topic
                ]
            }
        )
    }

    pub fn log_test_app2_update(deal_id: &str) -> Value {
        let test_app = Self::test_app2();
        let app_cid = test_app.encoded_cid();
        json!(
              {
                "address": deal_id,
                "topics": [
                  "0x0e85c04920a2349be7d0f03a765fa172e5dabc0a4a9fc47acb81c07ce8d260d0",
                ],
                "data": format!("0x{app_cid}"),
                "blockNumber": "0x300",
                "transactionHash": "0xb825edf7da59840ce838a9ed70aa0aa6c54c322ca5d6f0be4f070766e46ebbd8",
                "transactionIndex": "0xb",
                "blockHash": "0x34ba65babca6f1ef44da5f75c7bb4335c7b7484178a74003de5df139ac6551ed",
                "logIndex": "0x26",
                "removed": false
              }
        )
    }
}

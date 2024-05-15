use rand::Rng;
use serde_json::json;
use tracing::log;

use crate::utils::default::TX_RECEIPT_STATUS_OK;
use crate::utils::test_rpc_server::ServerHandle;
use crate::utils::TestApp;

pub fn random_tx() -> String {
    let mut rng = rand::thread_rng();
    let tx_vec: Vec<u8> = (0..32).map(|_| rng.gen()).collect::<_>();
    hex::encode(&tx_vec)
}

pub type ErrorReply = String;

#[derive(Clone, Debug)]
pub struct Deal {
    pub deal_id: String,
    pub app: Option<TestApp>,
    pub status: Option<String>,
}

impl Deal {
    pub fn ok(deal_id: &str, app: TestApp, status: &str) -> Self {
        Self {
            deal_id: deal_id.to_string(),
            app: Some(app),
            status: Some(status.to_string()),
        }
    }

    pub fn broken(deal_id: &str) -> Self {
        Self {
            deal_id: deal_id.to_string(),
            app: None,
            status: None,
        }
    }
}

fn get_compute_units(ids: &Vec<&String>) -> String {
    let cu_prefix = "aa3046a12a1aac6e840625e6";
    let mut result = format!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000{}", ids.len());
    for id in ids {
        // a unique cid is formed by appending the deal id to some prefix of existing cuid
        result.push_str(&format!("{cu_prefix}{id}000000000000000000000000{id}00000000000000000000000000000000000000000000000000000000000fffbc"));
    }
    result
}

#[derive(Clone)]
pub struct TxReceipt {
    pub tx_hash: String,
    pub status: String,
}

#[derive(Default, Clone)]
pub struct ChainReplies {
    pub deals: Vec<Deal>,
    // when None, reply with an error
    pub new_deals_tx_hashes: Vec<Option<String>>,
    pub new_deals_receipts: Vec<Option<TxReceipt>>,
}

impl ChainReplies {
    pub fn new(deals: Vec<Deal>, tx_hashes: Vec<String>) -> Self {
        Self {
            deals,
            new_deals_tx_hashes: tx_hashes.clone().into_iter().map(Some).collect(),
            new_deals_receipts: tx_hashes
                .into_iter()
                .map(|tx_hash| {
                    Some(TxReceipt {
                        tx_hash,
                        status: TX_RECEIPT_STATUS_OK.to_string(),
                    })
                })
                .collect(),
        }
    }
}

pub async fn play_chain(server: &mut ServerHandle, chain_replies: ChainReplies) {
    play_get_deals(server, &chain_replies.deals).await;
    for tx_hash in &chain_replies.new_deals_tx_hashes {
        play_register_worker_gen(server, tx_hash).await;
    }
    for receipts in &chain_replies.new_deals_receipts {
        play_tx_receipts_gen(server, receipts).await;
    }
}

pub async fn play_get_deals(server: &mut ServerHandle, deals: &Vec<Deal>) {
    let ids = deals.iter().map(|d| &d.deal_id).collect::<Vec<_>>();
    {
        // get compute units
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");
        assert_eq!(
            params[0].get("to").unwrap().as_str(),
            Some("market_contract")
        );

        server.send_response(Ok(json!(get_compute_units(&ids))));
    }


    // get app cid
    for _ in 0..deals.len() {
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");

        let requested_deal = params[0].get("to").unwrap().as_str().unwrap();
        let deal = deals.iter().find(|deal| requested_deal.ends_with(&deal.deal_id));
        assert!(deal.is_some(), "nox requested non-existent deal {requested_deal}, deals: {deals:?}");
        let deal = deal.unwrap();

        let reply = if let Some(ref app_cid) = deal.app {
            Ok(json!(app_cid.encoded_cid()))
        } else {
            Err(json!("no app cid provided"))
        };
        server.send_response(reply);
    }

    let are_apps_broken = deals.iter().any(|d| d.app.is_none());
    if are_apps_broken {
        log::debug!("do not expect status calls since app cids are broken");
        return;
    }
    // get deal status
    for _ in 0..deals.len() {
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");

        let requested_deal = params[0].get("to").unwrap().as_str().unwrap();
        let deal = deals.iter().find(|deal| requested_deal.ends_with(&deal.deal_id));
        assert!(deal.is_some(), "nox requested non-existent deal {requested_deal}, deals: {deals:?}");
        let deal = deal.unwrap();

        let reply = if let Some(ref status) = deal.status {
            Ok(json!(status))
        } else {
            Err(json!("no deal status provided"))
        };
        server.send_response(reply);
    }
}

pub async fn play_register_worker_gen(server: &mut ServerHandle, tx_hash: &Option<String>) {
    let estimate_gas = "0x5208";
    let max_priority_fee = "0x5208";
    let nonce = "0x20";

    {
        let result = json!({
          "hash": "0xcbe8d90665392babc8098738ec78009193c99d3cc872a6657e306cfe8824bef9",
          "parentHash": "0x15e767118a3e2d7545fee290b545faccd4a9eff849ac1057ce82cab7100c0c52",
          "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
          "miner": "0x0000000000000000000000000000000000000000",
          "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
          "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
          "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
          // logsBloom removed
          "difficulty": "0x0",
          "number": "0xa2",
          "gasLimit": "0x1c9c380",
          "gasUsed": "0x0",
          "timestamp": "0x65d88f76",
          "extraData": "0x",
          "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
          "nonce": "0x0000000000000000",
          "baseFeePerGas": "0x7",
          "totalDifficulty": "0x0",
          "uncles": [],
          "transactions": [],
          "size": "0x220"
        });
        let (method, _params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_getBlockByNumber");
        server.send_response(Ok(result));
    }
    {
        let (method, _params) = server.receive_request().await.unwrap();
        log::debug!("method: {method}, params: {_params:?}");
        assert_eq!(method, "eth_estimateGas");
        server.send_response(Ok(json!(estimate_gas)));
    }
    {
        let (method, _params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_maxPriorityFeePerGas");
        server.send_response(Ok(json!(max_priority_fee)));
    }
    {
        let (method, _params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_getTransactionCount");
        server.send_response(Ok(json!(nonce)));
    }

    {
        let (method, _params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_sendRawTransaction");
        let reply = if let Some(tx_hash) = tx_hash {
            Ok(json!(tx_hash))
        } else {
            Err(json!("no tx hash provided"))
        };
        server.send_response(reply);
    }
}

pub async fn play_tx_receipts_gen(server: &mut ServerHandle, receipt: &Option<TxReceipt>) {
    let (method, _params) = server.receive_request().await.unwrap();
    assert_eq!(method, "eth_getTransactionReceipt");
    let reply = if let Some(receipt) = receipt {
        Ok(json!({
            "blockNumber": "0x1",
            "transactionHash": receipt.tx_hash,
            "status": receipt.status
        }))
    } else {
        Err(json!("no receipt provided"))
    };
    server.send_response(reply);
}

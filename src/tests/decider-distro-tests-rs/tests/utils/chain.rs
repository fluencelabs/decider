/*
 * Nox Fluence Peer
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use rand::Rng;
use serde_json::{json, Value};
use tracing::log;

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
pub enum TxReceipt {
    Failed { hash: String },
    Ok { hash: String },
    Pending,
}

/// The configuration of replies from the chain for the deals
///
/// The amount of replies in each list depends on the decider state and test scenario
#[derive(Default, Clone)]
pub struct ChainReplies {
    // Replies for the `get_deals` builtin
    pub deals: Vec<Deal>,
    // Replies for the `register_worker` builtin
    // when None, reply with an RPC error
    pub new_deals_tx_hashes: Vec<Option<String>>,
    // Replies for the `get_tx_receipts` builtin
    // when None, reply with an RPC error
    pub new_deals_receipts: Vec<Option<TxReceipt>>,
}

impl ChainReplies {
    // Normal chain RPC reply sequence
    // tx_hashes is worker registration tx hashes and must be filled for each new deal for the
    // happy-path scenarios
    pub fn new(deals: Vec<Deal>, tx_hashes: Vec<String>) -> Self {
        Self {
            deals,
            new_deals_tx_hashes: tx_hashes.clone().into_iter().map(Some).collect(),
            new_deals_receipts: tx_hashes
                .into_iter()
                .map(|hash| Some(TxReceipt::Ok { hash }))
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

    for _ in 0..deals.len() {
        // Status Reply
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");

        let requested_deal = params[0].get("to").unwrap().as_str().unwrap();
        let deal = deals
            .iter()
            .find(|deal| requested_deal.ends_with(&deal.deal_id));
        assert!(
            deal.is_some(),
            "nox requested non-existent deal {requested_deal}, deals: {deals:?}"
        );
        let deal = deal.unwrap();

        let reply = if let Some(ref status) = deal.status {
            Ok(json!(status))
        } else {
            Err(json!("no deal status provided"))
        };
        server.send_response(reply);

        // App Cid Reply
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");

        let requested_deal = params[0].get("to").unwrap().as_str().unwrap();
        let deal = deals
            .iter()
            .find(|deal| requested_deal.ends_with(&deal.deal_id));
        assert!(
            deal.is_some(),
            "nox requested non-existent deal {requested_deal}, deals: {deals:?}"
        );
        let deal = deal.unwrap();

        let reply = if let Some(ref app_cid) = deal.app {
            Ok(json!(app_cid.encoded_cid()))
        } else {
            Err(json!("no app cid provided"))
        };
        server.send_response(reply);
    }

    /*

    // get app cid
    for _ in 0..deals.len() {
        let (method, params) = server.receive_request().await.unwrap();
        assert_eq!(method, "eth_call");

        let requested_deal = params[0].get("to").unwrap().as_str().unwrap();
        let deal = deals
            .iter()
            .find(|deal| requested_deal.ends_with(&deal.deal_id));
        assert!(
            deal.is_some(),
            "nox requested non-existent deal {requested_deal}, deals: {deals:?}"
        );
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
    }
    */
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
          // `logsBloom` is a part of the response, but is removed because it's big and useless
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

    // eth_getTransactionCount is now optional, so we may meet sendRawTransaction without getting
    // nonce
    {
        let (method, _params) = server.receive_request().await.unwrap();
        match method.as_ref() {
            "eth_sendRawTransaction" => {
                let reply = if let Some(tx_hash) = tx_hash {
                    Ok(json!(tx_hash))
                } else {
                    Err(json!("no tx hash provided"))
                };
                server.send_response(reply);
            }
            "eth_getTransactionCount" => {
                server.send_response(Ok(json!(nonce)));
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
            _ => panic!("unexpected method: {method}, expected eth_getTransactionCount or eth_sendRawTransaction"),
        }
    }
}

pub async fn play_tx_receipts_gen(server: &mut ServerHandle, receipt: &Option<TxReceipt>) {
    let (method, _params) = server.receive_request().await.unwrap();
    assert_eq!(method, "eth_getTransactionReceipt");

    let reply = match receipt {
        None => Err(json!("no receipt provided")),
        Some(receipt) => match receipt {
            TxReceipt::Failed { hash } => Ok(json!({
                "blockNumber": "0x1",
                "transactionHash": hash,
                "status": "0x0"
            })),
            TxReceipt::Ok { hash } => Ok(json!({
                "blockNumber": "0x1",
                "transactionHash": hash,
                "status": "0x1"
            })),
            TxReceipt::Pending => Ok(Value::Null),
        },
    };

    server.send_response(reply);
}

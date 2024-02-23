use crate::chain::chain_data::ChainData;
use crate::chain::event::deal_peer_removed::DealPeerRemovedData;
use crate::chain::log::Log;
use crate::curl::send_jsonrpc_batch;
use crate::jsonrpc::get_encoded_peer_id;
use crate::jsonrpc::get_logs::GetLogsReq;
use crate::jsonrpc::request::check_url;
use crate::jsonrpc::right_boundary::default_right_boundary;
use crate::jsonrpc::JsonRpcReq;
use marine_rs_sdk::marine;
use std::iter::zip;

#[derive(Debug)]
#[marine]
pub struct DealPeerRemovedReq {
    pub deal_id: String,
    pub left_boundary: String,
}

impl DealPeerRemovedReq {
    fn jsonrpc(&self, host_topic: String, idx: usize) -> JsonRpcReq<GetLogsReq> {
        let right_boundary = default_right_boundary(&self.left_boundary);
        let req = GetLogsReq {
            address: self.deal_id.clone(),
            topics: vec![DealPeerRemovedData::topic(), host_topic],
            from_block: self.left_boundary.clone(),
            to_block: right_boundary,
        };

        req.into_jsonrpc(idx as u32)
    }
}

#[derive(Debug)]
#[marine]
pub struct DealPeerRemovedResult {
    success: bool,
    error: Vec<String>,
    is_removed: bool,
    right_boundary: String,
    deal_id: String,
}

impl DealPeerRemovedResult {
    pub fn ok(right_boundary: String, deal_id: String, is_removed: bool) -> Self {
        Self {
            success: true,
            error: vec![],
            is_removed,
            right_boundary,
            deal_id,
        }
    }

    pub fn error(right_boundary: String, deal_id: String, err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            is_removed: false,
            right_boundary,
            deal_id,
        }
    }
}

#[derive(Debug)]
#[marine]
pub struct DealPeerRemovedBatchResult {
    result: Vec<DealPeerRemovedResult>,
    success: bool,
    error: Vec<String>,
}

impl DealPeerRemovedBatchResult {
    pub fn empty() -> Self {
        Self::ok(<_>::default())
    }

    pub fn ok(result: Vec<DealPeerRemovedResult>) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            result: vec![],
        }
    }
}

fn deal_peer_removed_req_batch(
    deals: &[DealPeerRemovedReq],
    host_topic: String,
) -> Vec<JsonRpcReq<GetLogsReq>> {
    deals
        .iter()
        .enumerate()
        .map(|(idx, deal)| deal.jsonrpc(host_topic.clone(), idx))
        .collect::<Vec<_>>()
}

#[marine]
pub fn poll_deal_peer_removed_batch(
    api_endpoint: &str,
    deals: Vec<DealPeerRemovedReq>,
) -> DealPeerRemovedBatchResult {
    if let Err(err) = check_url(api_endpoint) {
        return DealPeerRemovedBatchResult::error(err.to_string());
    }
    if deals.is_empty() {
        return DealPeerRemovedBatchResult::empty();
    }
    let host_topic = match get_encoded_peer_id() {
        Ok(host_topic) => host_topic,
        Err(err) => {
            return DealPeerRemovedBatchResult::error(err.to_string());
        }
    };
    let batch = deal_peer_removed_req_batch(&deals, host_topic);
    let responses = send_jsonrpc_batch::<GetLogsReq, Vec<Log>>(api_endpoint, batch);
    let mut responses = match responses {
        Err(err) => return DealPeerRemovedBatchResult::error(err.to_string()),
        Ok(r) => r,
    };
    // The JSON-RPC specification DOES NOT guarantee that the responses are in the same order as the
    // requests, so to correctly match the responses with the deal ids we need to sort the responses
    // in the asc order.
    responses.sort_by(|a, b| a.id.cmp(&b.id));

    let mut results = Vec::new();
    for (deal, result) in zip(deals, responses) {
        let to_block = default_right_boundary(&deal.left_boundary);
        match result.get_result() {
            Err(err) => {
                let result = DealPeerRemovedResult::error(to_block, deal.deal_id, err.to_string());
                results.push(result);
            }
            Ok(logs) => {
                let is_removed = !logs.is_empty();
                let result = DealPeerRemovedResult::ok(to_block, deal.deal_id, is_removed);
                results.push(result);
            }
        }
    }
    DealPeerRemovedBatchResult::ok(results)
}

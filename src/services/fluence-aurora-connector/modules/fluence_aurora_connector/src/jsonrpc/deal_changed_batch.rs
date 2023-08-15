use std::iter::zip;

use marine_rs_sdk::marine;

use crate::chain::deal_changed::parse_deal_changed;
use crate::chain::log::Log;
use crate::curl::send_jsonrpc_batch;
use crate::jsonrpc::deal_changed::{DealChangedResult, DealChangesReq};
use crate::jsonrpc::get_logs::GetLogsReq;
use crate::jsonrpc::request::check_url;
use crate::jsonrpc::right_boundary::default_right_boundary;
use crate::jsonrpc::JsonRpcReq;

#[marine]
pub struct MultipleDealsChanged {
    pub changes: Vec<DealChangedResult>,
    pub success: bool,
    pub error: Vec<String>,
}

impl MultipleDealsChanged {
    pub fn empty() -> Self {
        Self::ok(<_>::default())
    }

    pub fn ok(changes: Vec<DealChangedResult>) -> Self {
        Self {
            success: true,
            error: vec![],
            changes,
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            changes: vec![],
        }
    }
}

pub fn deal_changed_req_batch(deals: &[DealChangesReq]) -> Vec<JsonRpcReq<GetLogsReq>> {
    deals
        .iter()
        .enumerate()
        .map(|(idx, deal)| deal.jsonrpc(idx))
        .collect::<Vec<_>>()
}

#[marine]
pub fn poll_deal_changes(api_endpoint: &str, deals: Vec<DealChangesReq>) -> MultipleDealsChanged {
    if let Err(err) = check_url(api_endpoint) {
        return MultipleDealsChanged::error(err.to_string());
    }
    if deals.is_empty() {
        return MultipleDealsChanged::empty();
    }

    let batch = deal_changed_req_batch(&deals);
    let responses = send_jsonrpc_batch::<GetLogsReq, Vec<Log>>(api_endpoint, batch);
    let responses = match responses {
        Err(err) => return MultipleDealsChanged::error(err.to_string()),
        Ok(r) => r,
    };

    let mut updated_deals = Vec::new();

    for (deal, result) in zip(deals, responses) {
        let to_block = default_right_boundary(&deal.left_boundary);
        match result.get_result() {
            Err(err) => {
                let result = DealChangedResult::error(to_block, deal.deal_info, err.to_string());
                updated_deals.push(result);
            }
            Ok(result) => {
                let last_log = result.into_iter().filter(|deal| !deal.removed).last();
                let change = last_log.and_then(parse_deal_changed);
                if let Some(change) = change {
                    let result = DealChangedResult::ok(to_block, deal.deal_info, change);
                    updated_deals.push(result);
                }
            }
        }
    }

    MultipleDealsChanged::ok(updated_deals)
}

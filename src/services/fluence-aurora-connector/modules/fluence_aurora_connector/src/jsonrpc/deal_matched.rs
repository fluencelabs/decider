use marine_rs_sdk::marine;

use crate::chain::chain_data::ChainData;
use crate::chain::deal_matched::{DealMatched, Match};
use crate::chain::log::parse_logs;
use crate::jsonrpc::deal_changed::default_right_boundary;
use crate::jsonrpc::get_logs::get_logs;
use crate::jsonrpc::request::check_url;

#[marine]
pub struct MatchedResult {
    error: Vec<String>,
    success: bool,
    logs: Vec<DealMatched>,
    /// The response contains logs for blocks from `left_boundary` to `right_boundary`
    right_boundary: String,
}

impl MatchedResult {
    pub fn ok(logs: Vec<DealMatched>, right_boundary: String) -> Self {
        Self {
            success: true,
            error: vec![],
            logs,
            right_boundary,
        }
    }

    pub fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            logs: vec![],
            right_boundary: String::new(),
        }
    }
}

// TODO: How to set an upper limit for how many responses to return?
//       Don't see this functionallity in eth_getLogs
// TODO: need to restrict who can use this service to its spell
//
// `api_endpoint` -- api endpoint to poll (right now it's possible to pass any URL for emergency cases)
// `address`      -- address of the chain contract
// `left_boundary`   -- from which block to poll deals
#[marine]
pub fn poll_deal_matches(
    api_endpoint: String,
    address: String,
    left_boundary: String,
) -> MatchedResult {
    if let Err(err) = check_url(&api_endpoint) {
        return MatchedResult::error(err.to_string());
    }

    let right_boundary = default_right_boundary(&left_boundary);
    let result = get_logs(
        api_endpoint,
        address,
        left_boundary,
        right_boundary.clone(),
        Match::topic(),
    );
    match result {
        Err(err) => return MatchedResult::error(err.to_string()),
        Ok(logs) => {
            let created_deals = parse_logs::<Match, DealMatched>(logs);
            MatchedResult::ok(created_deals, right_boundary)
        }
    }
}

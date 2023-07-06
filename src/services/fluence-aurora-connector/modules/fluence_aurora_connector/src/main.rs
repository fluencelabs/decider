#![allow(dead_code)]
#![feature(assert_matches)]
#![feature(try_blocks)]

use std::iter::zip;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use thiserror::Error;

use jsonrpc::request;
use jsonrpc::request::*;

use crate::chain::chain_data::ChainData;
use crate::chain::deal_changed::*;
use crate::chain::deal_created::*;
use crate::chain::log::{parse_logs, Log};
use crate::curl::send_jsonrpc_batch;
use crate::jsonrpc::deal_changed::{
    deal_changed_req_batch, default_right_boundary, DealChangedResult, DealChangesReq,
    MultipleDealsChanged,
};
use crate::jsonrpc::deal_created::DealCreatedResult;
use crate::jsonrpc::get_logs::{get_logs, GetLogsReq};

mod chain;
mod config;
mod curl;
mod hex;
mod jsonrpc;
mod latest_block;
mod matches;
mod web3;

module_manifest!();

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    RequestError(#[from] request::RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpc::JsonRpcError),
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Trace)
        .build()
        .unwrap();
}

// TODO: How to set an upper limit for how many responses to return?
//       Don't see this functionallity in eth_getLogs
// TODO: need to restrict who can use this service to its spell
//
// `api_endpoint` -- api endpoint to poll (right now it's possible to pass any URL for emergency cases)
// `address`      -- address of the chain contract
// `left_boundary`   -- from which block to poll deals
#[marine]
pub fn poll_deal_created(
    api_endpoint: String,
    address: String,
    left_boundary: String,
) -> DealCreatedResult {
    if let Err(err) = check_url(&api_endpoint) {
        return DealCreatedResult::error(err.to_string());
    }

    let right_boundary = default_right_boundary(&left_boundary);
    let result = get_logs(
        api_endpoint,
        address,
        left_boundary,
        right_boundary.clone(),
        DealCreatedData::topic(),
    );
    match result {
        Err(err) => return DealCreatedResult::error(err.to_string()),
        Ok(logs) => {
            let created_deals = parse_logs::<DealCreatedData, DealCreated>(logs);
            DealCreatedResult::ok(created_deals, right_boundary)
        }
    }
}

#[marine]
pub fn poll_deal_changes(api_endpoint: String, deals: Vec<DealChangesReq>) -> MultipleDealsChanged {
    if let Err(err) = check_url(&api_endpoint) {
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

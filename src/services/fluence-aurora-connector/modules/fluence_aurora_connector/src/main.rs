#![allow(dead_code)]
#![feature(assert_matches)]
#![feature(try_blocks)]

mod deal;
mod jsonrpc;
mod request;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use std::collections::HashMap;
use thiserror::Error;

use deal::changed_cid::*;
use deal::created::*;
use deal::ChainData;
use deal::ChainEvent;
use request::*;

use jsonrpc::GetLogsResp;

module_manifest!();

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    RequestError(#[from] request::RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpc::JsonRpcError),
}

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub struct Net {
    /// Short name for the net. Can be used in `poll` calls.
    name: String,
    /// URL of the net
    url: String,
}

#[marine]
pub struct SupportedEvent {
    /// Name of the event
    name: String,
    /// Topic by which we poll the event
    topic: String,
}

/// Service configuration.
#[marine]
pub struct Env {
    /// List of allowed networks.
    nets: Vec<Net>,
    /// List of polled events with
    events: Vec<SupportedEvent>,
}

// TODO: allow owners to configure the service
#[marine]
pub fn get_env() -> Env {
    let nets = nets()
        .into_iter()
        .map(|(name, url)| Net {
            name: name.to_string(),
            url: url.to_string(),
        })
        .collect::<_>();
    let events = vec![
        SupportedEvent {
            name: DealCreated::EVENT_NAME.to_string(),
            topic: DealCreatedData::topic(),
        },
        SupportedEvent {
            name: DealChanged::EVENT_NAME.to_string(),
            topic: DealChangedData::topic(),
        },
    ];
    Env { nets, events }
}

// Nets we allow to poll.
fn nets() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("testnet", "https://aged-tiniest-scion.matic-testnet.quiknode.pro/08133c1e70a6ec1e7a75545a1254d85640a6251d/"),
        (
            "polygon-testnet",
            "https://endpoints.omniatech.io/v1/matic/mumbai/public",
        ),
        ("aurora-testnet", "https://testnet.aurora.dev"),
        // Note: cool for debugging, but do we want to leave it here?
        ("local", "http://localhost:8545"),
    ])
}

fn get_url(net: &str) -> Result<String, String> {
    nets()
        .get(net)
        .map(|x| String::from(*x))
        .ok_or_else(|| format!("unknown net: {}", net))
}

fn get_to_block(from_block: &str) -> String {
    let from_block = from_block.trim_start_matches("0x");
    let to_block = try {
        let from_block = u64::from_str_radix(from_block, 16).ok()?;
        from_block.checked_add(9999)?
    };
    match to_block {
        Some(to_block) => format!("{:#x}", to_block),
        None => "latest".to_string(),
    }
}

#[marine]
pub fn blocks_diff(from: String, to: String) -> u64 {
    let diff: Option<u64> = try {
        let from = from.trim_start_matches("0x");
        let from = u64::from_str_radix(from, 16).ok()?;

        let to = to.trim_start_matches("0x");
        let to = u64::from_str_radix(to, 16).ok()?;

        to.checked_sub(from)?
    };
    diff.unwrap_or(0)
}

#[marine]
pub struct BlockNumberResult {
    success: bool,
    result: String,
}

#[marine]
pub fn latest_block_number(net: String) -> BlockNumberResult {
    let url = match get_url(&net) {
        Err(_err) => {
            // TODO: right now we allow to use URL directly for emergency cases.
            // return DealCreatedResult::error(err);
            net
        }
        Ok(url) => url,
    };

    let result: Result<String, Error> = try {
        let result = get_block_number(url)?;
        log::debug!("request result: {:?}", result);
        result.get_result()?
    };
    match result {
        Ok(result) => BlockNumberResult { success: true, result },
        Err(err) => {
            log::warn!("can't get block number: {}", err.to_string());
            BlockNumberResult { success: false, result: String::new() }
        }
    }
}

#[marine]
pub struct DealCreatedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealCreated>,
    to_block: String,
}

impl DealCreatedResult {
    fn ok(result: Vec<DealCreated>, to_block: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
            to_block,
        }
    }

    fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
            to_block: String::new(),
        }
    }
}

// TODO: How to set an upper limit for how many responses to return?
//       Don't see this functionallity in eth_getLogs
// TODO: need to restrict who can use this service to its spell.
//
// `net` -- network type to poll (right now it's possible to pass any URL for emergency cases)
// `address` -- address of the deal contract
// `from_block` -- from which block to poll deals
#[marine]
pub fn poll_deals(net: String, address: String, from_block: String) -> DealCreatedResult {
    let to_block = get_to_block(&from_block);
    let result = poll(net, address, from_block, to_block.clone(), DealCreatedData::topic());
    match result {
        Err(err) => return DealCreatedResult::error(err.to_string()),
        Ok(deals) => {
            let changed_deals = parse_deals::<DealCreatedData, DealCreated>(deals);
            DealCreatedResult::ok(changed_deals, to_block)
        }
    }
}

#[marine]
pub struct DealChangedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealChanged>,
    to_block: String,
}

impl DealChangedResult {
    fn ok(result: Vec<DealChanged>, to_block: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
            to_block,
        }
    }

    fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
            to_block: String::new(),
        }
    }
}

// `net` -- network type to poll (right now it's possible to pass any URL for emergency cases)
// `address` -- address of the deal we are modifying
// `from_block` -- from which block to poll deals
#[marine]
pub fn poll_deal_change(net: String, address: String, from_block: String) -> DealChangedResult {
    let to_block = get_to_block(&from_block);
    let result = poll(net, address, from_block, to_block.clone(), DealChangedData::topic());
    match result {
        Err(err) => return DealChangedResult::error(err.to_string()),
        Ok(deals) => {
            let changed_deals = parse_deals::<DealChangedData, DealChanged>(deals);
            DealChangedResult::ok(changed_deals, to_block)
        }
    }
}

fn poll(
    net: String,
    address: String,
    from_block: String,
    to_block: String,
    topic: String,
) -> Result<Vec<GetLogsResp>, Error> {
    let url = match get_url(&net) {
        Err(_err) => {
            // TODO: right now we allow to use URL directly for emergency cases.
            // return DealCreatedResult::error(err);
            net
        }
        Ok(url) => url,
    };

    log::debug!("sending request to {}", url);
    let value = get_logs(url, address, vec![topic], from_block, to_block)?;
    log::debug!("request result: {:?}", value);
    let deals = value.get_result()?;
    Ok(deals)
}

fn parse_deals<U: ChainData, T: ChainEvent<U>>(deals: Vec<GetLogsResp>) -> Vec<T> {
    deals
        .into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| {
            log::debug!("Parse block {:?}", deal.block_number);
            let data = U::parse(&deal.data);
            match data {
                Err(err) => {
                    // Here we ignore blocks we cannot parse.
                    // Is it okay? We can't send warning
                    log::warn!(target: "connector",
                        "Cannot parse data of deal from block {}: {:?}",
                        deal.block_number,
                        err.to_string()
                    );
                    None
                }
                Ok(data) => Some(T::new(deal.block_number, data)),
            }
        })
        .collect()
}

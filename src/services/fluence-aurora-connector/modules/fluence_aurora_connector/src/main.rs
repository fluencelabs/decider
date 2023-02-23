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
        (
            "testnet",
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

#[marine]
pub struct DealCreatedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealCreated>,
}

impl DealCreatedResult {
    fn ok(result: Vec<DealCreated>) -> Self {
        Self {
            success: true,
            error: vec![],
            result: result,
        }
    }

    fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
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
    let result = poll(net, address, from_block, DealCreatedData::topic());
    match result {
        Err(err) => return DealCreatedResult::error(err.to_string()),
        Ok(deals) => {
            let changed_deals = parse_deals::<DealCreatedData, DealCreated>(deals);
            DealCreatedResult::ok(changed_deals)
        }
    }
}

#[marine]
pub struct DealChangedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealChanged>,
}

impl DealChangedResult {
    fn ok(result: Vec<DealChanged>) -> Self {
        Self {
            success: true,
            error: vec![],
            result: result,
        }
    }

    fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
        }
    }
}

// `net` -- network type to poll (right now it's possible to pass any URL for emergency cases)
// `address` -- address of the deal we are modifying
// `from_block` -- from which block to poll deals
#[marine]
pub fn poll_deal_change(net: String, address: String, from_block: String) -> DealChangedResult {
    let result = poll(net, address, from_block, DealChangedData::topic());
    match result {
        Err(err) => return DealChangedResult::error(err.to_string()),
        Ok(deals) => {
            let changed_deals = parse_deals::<DealChangedData, DealChanged>(deals);
            DealChangedResult::ok(changed_deals)
        }
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    RequestError(#[from] request::RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpc::JsonRpcError),
}

fn poll(
    net: String,
    address: String,
    from_block: String,
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
    let value = get_logs(url, address, vec![topic], from_block)?;
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

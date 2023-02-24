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

use deal::*;
use request::*;

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
    let events = vec![SupportedEvent {
        name: DealCreated::EVENT_NAME.to_string(),
        topic: DealCreated::topic(),
    }];
    Env { nets, events }
}

// Nets we allow to poll.
fn nets() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("testnet", "https://aged-tiniest-scion.matic-testnet.quiknode.pro/08133c1e70a6ec1e7a75545a1254d85640a6251d/"),
        ("polygon-testnet", "https://endpoints.omniatech.io/v1/matic/mumbai/public"),
        ("aurora-testnet", "https://testnet.aurora.dev"),
        // Note: cool for debugging, but do we want to leave it here?
        ("local", "http://localhost:8545"),
    ])
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

    let result = match get_block_number(url) {
        Err(err) => {
            log::debug!(target: "connector", "request error: {:?}", err);
            return BlockNumberResult { success: false, result: String::new() };
        },
        Ok(result) => result
    };
    log::debug!(target: "connector", "request result: {:?}", result);
    let result = match result.get_result() {
        Err(_) => { return BlockNumberResult { success: false, result: String::new() }; },
        Ok(result) => result
    };

    let hex_num = result.trim_start_matches("0x");
    if u64::from_str_radix(&hex_num, 16).is_err() {
        log::debug!(target: "connector", "{:?} isn't a hex number", result);
        return BlockNumberResult { success: false, result: String::new() };
    }

    BlockNumberResult { success: true, result }
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
#[marine]
pub fn poll_deals(net: String, address: String, from_block: String) -> DealCreatedResult {
    let url = match get_url(&net) {
        Err(_err) => {

            // TODO: right now we allow to use URL directly for emergency cases.
            // Would be nice to now
            // return DealCreatedResult::error(err);
            net
        }
        Ok(url) => url,
    };
    log::debug!("sending request to {}", url);
    let result = get_logs(url, address, vec![DealCreated::topic()], from_block);
    let value = match result {
        Err(err) => {
            return DealCreatedResult::error(err.to_string());
        }
        Ok(value) => value,
    };
    log::debug!("request result: {:?}", value);
    let deals = match value.get_result() {
        Err(err) => return DealCreatedResult::error(err.to_string()),
        Ok(deals) => deals,
    };

    let deals = deals
        .into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| {
            log::debug!("Parse block {:?}", deal.block_number);
            let data = parse_chain_deal_data(&deal.data);
            match data {
                Err(err) => {
                    // Here we ignore blocks we cannot parse.
                    // Is it okay? We can't send warning
                    log::warn!(
                        "Cannot parse data of deal from block {}: {:?}",
                        deal.block_number,
                        err.to_string()
                    );
                    None
                }
                Ok(data) => Some(DealCreated::new(deal.block_number, data)),
            }
        })
        .collect();

    DealCreatedResult::ok(deals)
}

fn get_url(net: &str) -> Result<String, String> {
    nets()
        .get(net)
        .map(|x| String::from(*x))
        .ok_or_else(|| format!("unknown net: {}", net))
}

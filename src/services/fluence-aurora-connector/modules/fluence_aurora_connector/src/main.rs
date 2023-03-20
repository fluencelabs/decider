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

use jsonrpc::{GetLogsReq, GetLogsResp};

module_manifest!();

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    RequestError(#[from] request::RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpc::JsonRpcError),
    #[error("unsupported network type: {0}")]
    NetworkTypeError(String),
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
    error: Vec<String>,
}

impl BlockNumberResult {
    fn ok(result: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
        }
    }

    fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: String::new(),
        }
    }
}

#[marine]
pub fn latest_block_number(net: String) -> BlockNumberResult {
    let url = match get_url(&net) {
        None => {
            return BlockNumberResult::error(Error::NetworkTypeError(net).to_string());
        }
        Some(url) => url,
    };

    let result = match get_block_number(url) {
        Err(err) => {
            log::debug!(target: "connector", "request error: {:?}", err);
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };
    log::debug!(target: "connector", "request result: {:?}", result);
    let result = match result.get_result() {
        Err(err) => {
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };

    let hex_num = result.trim_start_matches("0x");
    if u64::from_str_radix(&hex_num, 16).is_err() {
        log::debug!(target: "connector", "{:?} isn't a hex number", result);
        return BlockNumberResult::error(format!(
            "can't parse a block: {:?} isn't a hex number",
            result
        ));
    }
    BlockNumberResult::ok(result)
}

fn get_url(net: &str) -> Option<String> {
    nets().get(net).map(|x| String::from(*x))
}

fn hex_to_int(block: &str) -> Option<u64> {
    let block = block.trim_start_matches("0x");
    u64::from_str_radix(block, 16).ok()
}

fn int_to_hex(num: u64) -> String {
    format!("{:#x}", num)
}

fn get_to_block(from_block: &str) -> String {
    let to_block = try {
        let from_block = hex_to_int(from_block)?;
        from_block.checked_add(9999)?
    };
    match to_block {
        Some(to_block) => int_to_hex(to_block),
        None => "latest".to_string(),
    }
}

#[marine]
pub fn blocks_diff(from: String, to: String) -> u64 {
    let diff: Option<u64> = try {
        let from = hex_to_int(&from)?;
        let to = hex_to_int(&to)?;

        to.checked_sub(from)?
    };
    diff.unwrap_or(0)
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
    let result = poll(
        net,
        address,
        from_block,
        to_block.clone(),
        DealCreatedData::topic(),
    );
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
    deal_id: String,
}

impl DealChangedResult {
    fn ok(deal_id: String, result: Vec<DealChanged>, to_block: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
            to_block,
            deal_id,
        }
    }

    fn error(deal_id: String, err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
            to_block: String::new(),
            deal_id,
        }
    }
}

// `net` -- network type to poll (right now it's possible to pass any URL for emergency cases)
// `address` -- address of the deal we are modifying
// `from_block` -- from which block to poll deals
#[marine]
pub fn poll_deal_changed(net: String, deal_id: String, from_block: String) -> DealChangedResult {
    let address = format!("0x{}", deal_id);
    let to_block = get_to_block(&from_block);
    let result = poll(
        net,
        address,
        from_block,
        to_block.clone(),
        DealChangedData::topic(),
    );
    match result {
        Err(err) => return DealChangedResult::error(deal_id, err.to_string()),
        Ok(deals) => {
            let changed_deals = parse_deals::<DealChangedData, DealChanged>(deals);
            DealChangedResult::ok(deal_id, changed_deals, to_block)
        }
    }
}

#[derive(Debug)]
#[marine]
pub struct DealUpdate {
    deal_info: DealInfo,
    from_block: String,
}

#[derive(Debug)]
#[marine]
pub struct DealInfo {
    worker_id: String,
    //spell_id: String,
    deal_id: String,
}

#[marine]
pub struct DealUpdatedBatchResult {
    success: bool,
    /// optional error
    error: Vec<String>,
    /// optional result (present if success is true)
    result: Vec<DealChanged>,
    /// The request checked blocks from `from_block` to `to_block`
    to_block: String,
    /// Return deal info to be able to find which deal to update
    deal_info: DealInfo,
}

impl DealUpdatedBatchResult {
    fn ok(to_block: String, deal_info: DealInfo, update: DealChanged) -> Self {
        Self {
            success: true,
            error: vec![],
            result: vec![update],
            to_block,
            deal_info,
        }
    }

    fn error(to_block: String, deal_info: DealInfo, err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            result: vec![],
            to_block,
            deal_info,
        }
    }
}

#[marine]
pub struct DealsUpdatedBatchResult {
    result: Vec<DealUpdatedBatchResult>,
    success: bool,
    error: Vec<String>,
}

impl DealsUpdatedBatchResult {
    fn ok(result: Vec<DealUpdatedBatchResult>) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
        }
    }

    fn error(err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            result: vec![],
        }
    }
}

#[marine]
pub fn poll_deals_latest_update_batch(net: String, deals: Vec<DealUpdate>) -> DealsUpdatedBatchResult {
    let url = match get_url(&net) {
        None => {
            return DealsUpdatedBatchResult::error(Error::NetworkTypeError(net).to_string());
        }
        Some(url) => url,
    };

    if deals.is_empty() {
        return DealsUpdatedBatchResult::ok(Vec::new());
    }

    let mut updated_deals = Vec::new();
    let reqs = deals
        .iter()
        .enumerate()
        .map(|(idx, deal)| {
            let to_block = get_to_block(&deal.from_block);
            let address = format!("0x{}", deal.deal_info.deal_id);
            let req = GetLogsReq {
                address,
                topics: vec![DealChangedData::topic()],
                from_block: deal.from_block.clone(),
                to_block,
            };
            req.to_jsonrpc(idx as u32)
        })
        .collect::<Vec<_>>();
    let result = get_logs_batch(url, reqs);
    match result {
        Err(err) => {
            return DealsUpdatedBatchResult::error(err.to_string());
        }
        Ok(results) => {
            for (deal, result) in std::iter::zip(deals, results) {
                let to_block = get_to_block(&deal.from_block);
                match result.get_result() {
                    Err(err) => {
                        let result =
                            DealUpdatedBatchResult::error(to_block, deal.deal_info, err.to_string());
                        updated_deals.push(result);
                    },
                    Ok(result) => {
                        let latest_update: Option<DealChanged> = try {
                            let update = result.into_iter().filter(|deal| !deal.removed).last()?;
                            parse_deal::<DealChangedData, DealChanged>(update)?
                        };
                        if let Some(update) = latest_update {
                            let result = DealUpdatedBatchResult::ok(to_block, deal.deal_info, update);
                            updated_deals.push(result);
                        }
                    }
                }
            }
        }
    }

    DealsUpdatedBatchResult::ok(updated_deals)
}

/*
for deal in deals {
    let to_block = get_to_block(&deal.from_block);
    let address = format!("0x{}", deal.deal_info.deal_id);
    let result = poll(
        net.clone(),
        address,
        deal.from_block,
        to_block.clone(),
        DealChangedData::topic(),
    );
    match result {
        Err(err) => {
            let result =
                DealUpdatedBatchResult::error(to_block, deal.deal_info, err.to_string());
            results.push(result);
        }
        Ok(updates) => {
            let parsed_latest_update = try {
                let update = updates.into_iter().filter(|deal| !deal.removed).last()?;
                parse_deal::<DealChangedData, DealChanged>(update)?
            };

            // the last element of the list is the latest deal update
            if let Some(update) = parsed_latest_update {
                let result = DealUpdatedBatchResult::ok(to_block, deal.deal_info, update);
                results.push(result);
            }
        }
    };
}
*/

fn poll(
    net: String,
    address: String,
    from_block: String,
    to_block: String,
    topic: String,
) -> Result<Vec<GetLogsResp>, Error> {
    let url = match get_url(&net) {
        None => {
            return Err(Error::NetworkTypeError(net));
        }
        Some(url) => url,
    };

    log::debug!("sending request to {}", url);
    let value = get_logs(url, address, vec![topic], from_block, to_block)?;
    log::debug!("request result: {:?}", value);
    let deals = value.get_result()?;
    Ok(deals)
}

fn parse_deal<U: ChainData, T: ChainEvent<U>>(deal: GetLogsResp) -> Option<T> {
    log::debug!("Parse block {:?}", deal.block_number);
    match U::parse(&deal.data) {
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
        Ok(data) => {
            let block_number = hex_to_int(&deal.block_number)?;
            let next_block_number = int_to_hex(block_number + 1);
            Some(T::new(next_block_number, deal.block_number, data))
        }
    }
}

fn parse_deals<U: ChainData, T: ChainEvent<U>>(deals: Vec<GetLogsResp>) -> Vec<T> {
    deals
        .into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| parse_deal::<U, T>(deal))
        .collect()
}

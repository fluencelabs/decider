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

/// Service configuration.
#[marine]
pub struct Config {
    /// List of allowed networks.
    nets: Vec<Net>,
}

// TODO: allow owners to configure the service
#[marine]
pub fn get_config() -> Config {
    let nets = nets()
        .into_iter()
        .map(|(name, url)| Net {
            name: name.to_string(),
            url: url.to_string(),
        })
        .collect::<_>();
    Config { nets }
}

// Nets we allow to poll.
fn nets() -> HashMap<&'static str, &'static str> {
    HashMap::from([("testnet", "https://testnet.aurora.dev")])
}

#[marine]
pub struct DealResult {
    error: Vec<String>,
    success: bool,
    result: Vec<Deal>,
}

impl DealResult {
    fn ok(result: Vec<Deal>) -> Self {
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
// TODO: add url as a parameter?
// TODO: need to restrict who can use this service to its spell.
#[marine]
pub fn poll_deals(
    net: String,
    address: String,
    topics: Vec<String>,
    from_block: String,
) -> DealResult {
    let url = match get_url(net) {
        Err(err) => {
            return DealResult::error(err);
        }
        Ok(url) => url,
    };
    let result = send_request(url, address, topics, from_block);
    let value = match result {
        Err(err) => {
            return DealResult::error(err.to_string());
        }
        Ok(value) => value,
    };
    log::debug!("{:?}", value);
    let deals = match value.get_result() {
        Err(err) => return DealResult::error(err.to_string()),
        Ok(deals) => deals,
    };

    let deals = deals
        .into_iter()
        .map(|deal| {
            let data = parse_chain_deal_data(deal.data);
            Deal::new(deal.block_number, data)
        })
        .collect();

    DealResult::ok(deals)
}

fn get_url(net: String) -> Result<String, String> {
    nets()
        .get(net.as_str())
        .map(|x| String::from(*x))
        .ok_or_else(|| format!("unknown net: {}", net))
}

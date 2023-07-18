use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;

use crate::chain::chain_data::EventField::NotIndexed;
use crate::chain::chain_data::{ChainData, EventField, LogParseError};
use crate::chain::chain_event::ChainEvent;
use crate::chain::data_tokens::next_opt;
use crate::chain::log::{parse_log, Log};

/// Corresponding Solidity type:
/// ```solidity
/// event NewAppCID(string appCID);
/// ```
#[derive(Debug)]
#[marine]
pub struct DealChangedData {
    /// New CID for the deal
    app_cid: String,
}

#[derive(Debug)]
#[marine]
pub struct DealChanged {
    block_number: String,
    info: DealChangedData,
}

impl DealChanged {
    pub const EVENT_NAME: &str = "NewAppCID";
}

impl ChainData for DealChangedData {
    fn event_name() -> &'static str {
        DealChanged::EVENT_NAME
    }

    fn signature() -> Vec<EventField> {
        vec![
            NotIndexed(ParamType::String), // appCID
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, LogParseError> {
        let app_cid = next_opt(data_tokens, "app_cid", |t| t.into_string())?;
        Ok(DealChangedData { app_cid })
    }
}

impl ChainEvent<DealChangedData> for DealChanged {
    fn new(block_number: String, info: DealChangedData) -> Self {
        Self { block_number, info }
    }
}

pub fn parse_deal_changed(log: Log) -> Option<DealChanged> {
    // TODO: should we communicate these failures to Aqua code?
    parse_log::<DealChangedData, DealChanged>(log).ok()
}

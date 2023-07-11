use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;

use crate::chain::chain_data::{ChainData, DealParseError};
use crate::chain::chain_event::ChainEvent;
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
    fn topic() -> String {
        let sig = Self::signature();
        let hash = ethabi::long_signature(DealChanged::EVENT_NAME, &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
    }

    fn signature() -> Vec<ParamType> {
        vec![
            ParamType::String, // appCID
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(tokens: Vec<Token>) -> Result<Self, DealParseError> {
        let deal_data: Option<DealChangedData> = try {
            let app_cid = tokens.into_iter().next()?.into_string()?;
            DealChangedData { app_cid }
        };
        deal_data.ok_or_else(|| DealParseError::SignatureMismatch(Self::signature()))
    }
}

impl ChainEvent<DealChangedData> for DealChanged {
    fn new(block_number: String, info: DealChangedData) -> Self {
        Self { block_number, info }
    }
}

pub fn parse_deal_changed(log: Log) -> Option<DealChanged> {
    parse_log::<DealChangedData, DealChanged>(log).ok()
}

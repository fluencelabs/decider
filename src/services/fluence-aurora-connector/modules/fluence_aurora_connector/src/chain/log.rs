use serde::{Deserialize, Serialize};

use crate::chain::chain_data::{parse_chain_data, ChainData, DealParseError};
use crate::chain::chain_event::ChainEvent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    // Log arguments
    pub data: String,
    // The block number that contains this log
    pub block_number: String,
    // true when the log was removed, due to a chain reorganization. false if its a valid log.
    #[serde(default)]
    pub removed: bool,
    pub topics: Vec<String>,
}

pub fn parse_logs<U: ChainData, T: ChainEvent<U>>(logs: Vec<Log>) -> Vec<T> {
    logs.into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| parse_log::<U, T>(deal).ok())
        .collect()
}

pub fn parse_log<U: ChainData, T: ChainEvent<U>>(deal: Log) -> Result<T, DealParseError> {
    log::debug!("Parse log from block {:?}", deal.block_number);
    let result: Result<_, DealParseError> = try {
        let tokens = parse_chain_data(&deal.data, U::signature())?;
        let log = U::parse(tokens)?;
        T::new(deal.block_number.clone(), log)
    };

    if let Err(e) = result.as_ref() {
        log::warn!(target: "connector",
            "Cannot parse deal log from block {}: {:?}",
            deal.block_number,
            e.to_string()
        );
    }

    result
}

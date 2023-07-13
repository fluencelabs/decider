use serde::{Deserialize, Serialize};

use crate::chain::chain_data::ChainData;
use crate::chain::chain_event::ChainEvent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    // Log arguments
    pub data: String,
    // The block number that contains this log
    pub block_number: String,
    // true when the log was removed, due to a chain reorganization. false if its a valid log.
    pub removed: bool,
}

pub fn parse_logs<U: ChainData, T: ChainEvent<U>>(logs: Vec<Log>) -> Vec<T> {
    logs.into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| parse_log::<U, T>(deal))
        .collect()
}

pub fn parse_log<U: ChainData, T: ChainEvent<U>>(log: Log) -> Option<T> {
    log::debug!("Parse log from block {:?}", log.block_number);
    match U::parse(&log.data) {
        Err(err) => {
            // Here we ignore blocks we cannot parse.
            // Is it okay? We can't send warning
            log::warn!(target: "connector",
                "Cannot parse deal log from block {}: {:?}",
                log.block_number,
                err.to_string()
            );
            None
        }
        Ok(data) => Some(T::new(log.block_number, data)),
    }
}

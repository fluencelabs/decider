use serde::{Deserialize, Serialize};

use crate::chain::chain_data::{parse_chain_data, ChainData};
use crate::chain::chain_event::ChainEvent;
use crate::hex::{hex_to_int, int_to_hex};

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

pub fn parse_log<U: ChainData, T: ChainEvent<U>>(deal: Log) -> Option<T> {
    log::debug!("Parse log from block {:?}", deal.block_number);
    let tokens = parse_chain_data(&deal.data, U::signature());
    match tokens.and_then(U::parse) {
        Err(err) => {
            // Here we ignore blocks we cannot parse.
            // Is it okay? We can't send warning
            log::warn!(target: "connector",
                "Cannot parse deal log from block {}: {:?}",
                deal.block_number,
                err.to_string()
            );
            None
        }
        Ok(data) => Some(T::new(deal.block_number, data)),
    }
}

use crate::chain::chain_data::ChainData;
use crate::chain::chain_event::ChainEvent;
use crate::hex::{hex_to_int, int_to_hex};
use serde::{Deserialize, Serialize};

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
    log::debug!("Parse block {:?}", deal.block_number);
    match U::parse(&deal.data) {
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
        Ok(data) => {
            let block_number = hex_to_int(&deal.block_number)?;
            let next_block_number = int_to_hex(block_number + 1);
            Some(T::new(next_block_number, deal.block_number, data))
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::chain::chain_data::EventField::{Indexed, NotIndexed};
use crate::chain::chain_data::LogParseError::{MissingToken, MissingTopic};
use crate::chain::chain_data::{parse_chain_data, ChainData, LogParseError};
use crate::chain::chain_event::ChainEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// Parse Event Log to specified DTO
///
/// Logs consist of data fields, much like ADT. Fields can indexed and not indexed.
///
/// Data for indexed fields is encoded in 'log.topics', starting from 1th topic, i.e. 0th is skipped
/// Data for non indexed fields is encoded in 'log.data'.
///
/// Indexed and non indexed data fields can be interleaved.
/// That forces a certain parsing scheme, which is implemented below.
pub fn parse_log<U: ChainData, T: ChainEvent<U>>(log: Log) -> Result<T, LogParseError> {
    log::debug!("Parse log from block {:?}", log.block_number);
    let result: Result<_, LogParseError> = try {
        // event log signature, i.e. data field types
        let signature = U::signature();
        // gather data types for non indexed ("indexless") fields
        let indexless = signature
            .clone()
            .into_iter()
            .filter_map(|t| match t {
                NotIndexed(t) => Some(t),
                Indexed(_) => None,
            })
            .collect::<Vec<_>>();
        // parse all non indexed fields to tokens
        let indexless = parse_chain_data(log.data, &indexless)?;

        // iterate through data field types (signature), and take
        // data `Token` from either 'indexless' or 'topics'
        let mut indexless = indexless.into_iter();
        // skip first topic, because it contains actual topic, and not indexed data field
        let mut topics = log.topics.into_iter().skip(1);
        // accumulate tokens here
        let mut tokens = vec![];
        for (position, event_field) in signature.into_iter().enumerate() {
            match event_field {
                NotIndexed(_) => {
                    // take next token for non indexed data field
                    let token = indexless.next().ok_or(MissingToken {
                        position,
                        event_field,
                    })?;
                    tokens.push(token);
                }
                ef @ Indexed(_) => {
                    let topic = topics.next().ok_or(MissingTopic {
                        position,
                        event_field: ef.clone(),
                    })?;
                    // parse indexed field to token one by one
                    let parsed = parse_chain_data(topic, &[ef.clone().param_type()])?;
                    debug_assert!(parsed.len() == 1, "parse of an indexed event fields yielded several tokens, expected a single one");
                    let token = parsed.into_iter().next().ok_or(MissingToken {
                        position,
                        event_field: ef,
                    })?;
                    tokens.push(token)
                }
            }
        }

        if tokens.is_empty() {
            return Err(LogParseError::Empty);
        }

        let block_number = log.block_number.clone();
        println!("data tokens: {:?}", tokens);
        let log = U::parse(&mut tokens.into_iter())?;
        T::new(block_number, log)
    };

    if let Err(e) = result.as_ref() {
        log::warn!(target: "connector",
            "Cannot parse deal log from block {}: {:?}",
            log.block_number,
            e.to_string()
        );
    }

    result
}

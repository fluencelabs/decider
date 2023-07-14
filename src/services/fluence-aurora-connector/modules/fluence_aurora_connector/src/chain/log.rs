use serde::{Deserialize, Serialize};

use crate::chain::chain_data::DealParseError::{MissingToken, MissingTopic};
use crate::chain::chain_data::EventField::{Indexed, NotIndexed};
use crate::chain::chain_data::{parse_chain_data, ChainData, DealParseError};
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

pub fn parse_log<U: ChainData, T: ChainEvent<U>>(deal: Log) -> Result<T, DealParseError> {
    log::debug!("Parse log from block {:?}", deal.block_number);
    let result: Result<_, DealParseError> = try {
        let signature = U::signature();
        let indexless = signature
            .clone()
            .into_iter()
            .filter_map(|t| match t {
                NotIndexed(t) => Some(t),
                Indexed(_) => None,
            })
            .collect::<Vec<_>>();
        let indexless = parse_chain_data(&deal.data, &indexless)?;

        let mut indexless = indexless.into_iter();
        let mut topics = deal.topics.into_iter().skip(1);
        let mut tokens = vec![];
        for (position, event_field) in signature.into_iter().enumerate() {
            match event_field {
                NotIndexed(_) => {
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
                    let parsed = parse_chain_data(&topic, &[ef.clone().param_type()])?;
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
            return Err(DealParseError::Empty);
        }

        let log = U::parse(&mut tokens.into_iter())?;
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

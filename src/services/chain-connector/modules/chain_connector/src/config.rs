use marine_rs_sdk::marine;

use crate::chain::chain_data::ChainData;
use crate::chain::event::deal_changed::{DealChanged, DealChangedData};
use crate::chain::event::deal_matched::{DealMatched, Match};
use crate::chain::event::deal_peer_removed::{DealPeerRemoved, DealPeerRemovedData};

#[marine]
pub struct SupportedEvent {
    /// Name of the event
    name: String,
    /// Topic by which we poll the event
    topic: String,
}

/// Service configuration
#[marine]
pub struct Env {
    /// List of polled events with topics
    events: Vec<SupportedEvent>,
}

#[marine]
pub fn get_env() -> Env {
    let events = vec![
        SupportedEvent {
            name: DealChanged::EVENT_NAME.to_string(),
            topic: DealChangedData::topic(),
        },
        SupportedEvent {
            name: DealMatched::EVENT_NAME.to_string(),
            topic: Match::topic(),
        },
        SupportedEvent {
            name: DealPeerRemoved::EVENT_NAME.to_string(),
            topic: DealPeerRemovedData::topic(),
        },
    ];
    Env { events }
}

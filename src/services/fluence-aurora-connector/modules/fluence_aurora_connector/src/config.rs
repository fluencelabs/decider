use crate::chain::deal_changed::{DealChanged, DealChangedData};
use crate::chain::deal_created::{DealCreated, DealCreatedData};
use marine_rs_sdk::marine;

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
            name: DealCreated::EVENT_NAME.to_string(),
            topic: DealCreatedData::topic(),
        },
        SupportedEvent {
            name: DealChanged::EVENT_NAME.to_string(),
            topic: DealChangedData::topic(),
        },
    ];
    Env { events }
}

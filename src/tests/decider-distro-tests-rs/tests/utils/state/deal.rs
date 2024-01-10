use crate::utils::spell;
use connected_client::ConnectedClient;
use eyre::WrapErr;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct DealState {
    pub left_boundary: String,
}

pub async fn get_deal_state(client: &mut ConnectedClient, deal_id: &str) -> DealState {
    let result = spell::get_string(client, "decider", deal_id)
        .await
        .wrap_err("getting deal state")
        .unwrap();
    assert!(!result.absent, "no state for deal {}", deal_id);
    assert!(
        result.success,
        "can't get state for deal {}: {}",
        deal_id, result.error
    );
    serde_json::from_str::<DealState>(&result.value)
        .wrap_err("parse deal_state")
        .unwrap()
}

pub async fn get_deal_removed_state(client: &mut ConnectedClient, deal_id: &str) -> DealState {
    let key = format!("removed_state:0x{deal_id}");
    println!("key: {key}");
    let result = spell::get_string(client, "decider", &key)
        .await
        .wrap_err("getting deal state")
        .unwrap();
    assert!(!result.absent, "no state for deal {}", deal_id);
    assert!(
        result.success,
        "can't get state for deal {}: {}",
        deal_id, result.error
    );
    serde_json::from_str::<DealState>(&result.value)
        .wrap_err("parse deal_state")
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct JoinedDeal {
    pub deal_id: String,
    pub worker_id: String,
}

pub async fn get_joined_deals(client: &mut ConnectedClient) -> Vec<JoinedDeal> {
    let deals = spell::list_get_strings(client, "decider", "joined_deals")
        .await
        .unwrap();
    assert!(deals.success, "empty list of joined_deals: {}", deals.error);
    deals
        .value
        .iter()
        .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
        .collect()
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum FailedDealPayload {
    InstallationFailed { log: Value },
    TxFailed { tx_hash: Vec<String> },
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FailedDeal {
    deal_id: String,
    message: String,
    payload: FailedDealPayload,
}

pub async fn get_failed_deals(client: &mut ConnectedClient) -> Vec<FailedDeal> {
    let deals = spell::list_get_strings(client, "decider", "failed_deals")
        .await
        .unwrap();
    assert!(deals.success, "can't receive failed_deals: {}", deals.error);
    deals
        .value
        .iter()
        .map(|s| serde_json::from_str::<FailedDeal>(s))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

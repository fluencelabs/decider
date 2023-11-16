use crate::utils;
use connected_client::ConnectedClient;
use eyre::WrapErr;
use fluence_spell_dtos::value::{StringListValue, StringValue};
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct DealState {
    pub left_boundary: String,
}

pub async fn get_deal_state(client: &mut ConnectedClient, deal_id: &String) -> DealState {
    let mut result = utils::execute(
        client,
        r#"
            (call relay ("decider" "get_string") [deal_id] deal_state)
        "#,
        "deal_state",
        hashmap! {
            "deal_id" => json!(deal_id)
        },
    )
    .await
    .unwrap();
    let str = serde_json::from_value::<StringValue>(result.remove(0))
        .wrap_err("get deal_state")
        .unwrap();
    assert!(!str.absent, "no state for deal {}", deal_id);
    assert!(
        str.success,
        "can't get state for deal {}: {}",
        deal_id, str.error
    );
    serde_json::from_str::<DealState>(&str.str)
        .wrap_err("parse deal_state")
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct JoinedDeal {
    pub deal_id: String,
    pub worker_id: String,
}

pub fn parse_joined_deals(deals: Value) -> Vec<JoinedDeal> {
    let deals = serde_json::from_value::<StringListValue>(deals).unwrap();
    assert!(deals.success);
    deals
        .strings
        .iter()
        .map(|deal| serde_json::from_str::<JoinedDeal>(deal).unwrap())
        .collect()
}

pub async fn get_joined_deals(mut client: &mut ConnectedClient) -> Vec<JoinedDeal> {
    let mut deals = utils::execute(
        &mut client,
        r#"
            (call relay ("decider" "list_get_strings") ["joined_deals"] deals)
        "#,
        "deals",
        hashmap! {},
    )
    .await
    .unwrap();
    parse_joined_deals(deals.remove(0))
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

pub async fn get_failed_deals(mut client: &mut ConnectedClient) -> Vec<FailedDeal> {
    let mut deals = utils::execute(
        &mut client,
        r#"
            (call relay ("decider" "list_get_strings") ["failed_deals"] deals)
        "#,
        "deals",
        hashmap! {},
    )
    .await
    .unwrap();
    let lst = serde_json::from_value::<StringListValue>(deals.remove(0)).unwrap();
    assert!(lst.success, "can't receive failed_deals: {}", lst.error);
    lst.strings
        .iter()
        .map(|s| serde_json::from_str::<FailedDeal>(s))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

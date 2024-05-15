use serde::Deserialize;

use connected_client::ConnectedClient;

use crate::utils::spell;

#[derive(Deserialize, Debug)]
pub struct JoinedDeal {
    pub deal_id: String,
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

pub async fn remove_joined_deal(client: &mut ConnectedClient, deal_id: &str) -> eyre::Result<()> {
    let result = spell::list_remove_string(client, "decider", "joined_deals", deal_id)
        .await
        .unwrap();
    result
        .success
        .then(|| ())
        .ok_or_else(|| eyre::eyre!("failed to remove deal: {}", result.error))
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FailedDeal {
    pub deal_id: String,
    message: String,
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

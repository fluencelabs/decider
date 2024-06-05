use crate::utils::spell;
use connected_client::ConnectedClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkerTxInfo {
    pub deal_id: String,
    pub tx_hash: String,
}

pub async fn get_txs(mut client: &mut ConnectedClient) -> Vec<WorkerTxInfo> {
    let deal_txs = spell::list_get_strings(&mut client, "decider", "subnet_registration_txs")
        .await
        .unwrap();
    assert!(
        deal_txs.success,
        "can't receive `subnet_registration_txs`: {}",
        deal_txs.error
    );

    let mut txs = Vec::new();
    for deal_id in deal_txs.value {
        let tx_hash = spell::get_string(&mut client, "decider", &format!("tx_hash:{deal_id}")).await.unwrap();
        assert!(tx_hash.success, "can't get tx_hash:{deal_id}: {tx_hash:?}");
        txs.push(WorkerTxInfo { deal_id, tx_hash: tx_hash.value })
    }
    txs
}

use serde::Deserialize;
use connected_client::ConnectedClient;
use crate::utils::spell;

#[derive(Deserialize, Debug)]
pub struct WorkerTxInfo {
    pub deal_id: String,
    pub tx_hash: String,
}

pub async fn get_txs(mut client: &mut ConnectedClient) -> Vec<WorkerTxInfo> {
    let txs = spell::list_get_strings(&mut client, "decider", "worker_registration_txs")
        .await
        .unwrap();
    assert!(
        txs.success,
        "can't receive `worker_registration_txs`: {}",
        txs.error
    );
    txs.value
        .iter()
        .map(|tx| serde_json::from_str::<WorkerTxInfo>(tx).unwrap())
        .collect::<Vec<_>>()
}

#[derive(Deserialize, Debug)]
pub struct WorkerTxStatus {
    pub tx_info: WorkerTxInfo,
    pub status: String,
}

pub async fn get_txs_statuses(mut client: &mut ConnectedClient) -> Vec<WorkerTxStatus> {
    let txs = spell::list_get_strings(&mut client, "decider", "worker_registration_txs_statuses")
        .await
        .unwrap();
    assert!(
        txs.success,
        "can't receive `worker_registration_txs_statuses`: {}",
        txs.error
    );
    txs.value
        .iter()
        .map(|tx| serde_json::from_str::<WorkerTxStatus>(tx).unwrap())
        .collect::<Vec<_>>()
}

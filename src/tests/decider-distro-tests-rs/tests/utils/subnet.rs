use crate::utils;
use connected_client::ConnectedClient;
use fluence_spell_dtos::value::StringListValue;
use maplit::hashmap;
use serde::Deserialize;

// atm the we don't use some fields in the tests, but will do in future
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WorkerTxInfo {
    deal_id: String,
    tx_hash: String,
}

pub async fn get_txs(mut client: &mut ConnectedClient) -> Vec<WorkerTxInfo> {
    let mut result = utils::execute(
        &mut client,
        r#"
            (call relay ("decider" "list_get_strings") ["worker_registration_txs"] txs)
        "#,
        "txs",
        hashmap! {},
    )
    .await
    .unwrap();
    let txs = serde_json::from_value::<StringListValue>(result.remove(0)).unwrap();
    assert!(
        txs.success,
        "can't receive `worker_registration_txs`: {}",
        txs.error
    );
    txs.strings
        .iter()
        .map(|tx| serde_json::from_str::<WorkerTxInfo>(tx).unwrap())
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WorkerTxStatus {
    tx_info: WorkerTxInfo,
    status: String,
}

pub async fn get_txs_statuses(mut client: &mut ConnectedClient) -> Vec<WorkerTxStatus> {
    let mut result = utils::execute(
        &mut client,
        r#"
            (call relay ("decider" "list_get_strings") ["worker_registration_txs_statuses"] txs)
        "#,
        "txs",
        hashmap! {},
    )
    .await
    .unwrap();
    let txs = serde_json::from_value::<StringListValue>(result.remove(0)).unwrap();
    assert!(
        txs.success,
        "can't receive `worker_registration_txs_statuses`: {}",
        txs.error
    );
    txs.strings
        .iter()
        .map(|tx| serde_json::from_str::<WorkerTxStatus>(tx).unwrap())
        .collect::<Vec<_>>()
}

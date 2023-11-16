use crate::utils;
use connected_client::ConnectedClient;
use fluence_spell_dtos::value::StringValue;
use maplit::hashmap;
use serde_json::json;

pub async fn get_worker_app_cid(client: &mut ConnectedClient, worker_id: &String) -> String {
    let mut result = utils::execute(
        client,
        r#"
        (seq
            (call relay ("op" "noop") [])
            (call worker_id ("worker-spell" "get_string") ["worker_def_cid"] cid)
        )
        "#,
        "cid",
        hashmap! {
            "worker_id" => json!(worker_id),
        },
    )
    .await
    .unwrap();
    let result = serde_json::from_value::<StringValue>(result.remove(0)).unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");
    serde_json::from_str::<String>(&result.str).unwrap()
}

pub async fn get_worker(mut client: &mut ConnectedClient, deal: &str) -> Vec<String> {
    let mut worker = utils::execute(
        &mut client,
        r#"
            (call relay ("worker" "get_worker_id") [dealid] worker)
        "#,
        "worker",
        hashmap! {
            "dealid" => json!(format!("0x{deal}"))
        },
    )
    .await
    .unwrap();
    serde_json::from_value::<Vec<String>>(worker.remove(0)).unwrap()
}

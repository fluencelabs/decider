use eyre::WrapErr;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::json;

use connected_client::ConnectedClient;

use crate::utils;
use crate::utils::spell;

pub async fn get_worker_app_cid(client: &mut ConnectedClient, worker_id: &String) -> String {
    let result = spell::get_string_on(client, worker_id, "worker-spell", "h_worker_def_cid")
        .await
        .wrap_err("get_worker_app_cid failed")
        .unwrap();
    assert!(!result.absent, "worker-spell doesn't have worker_def_cid");
    serde_json::from_str::<String>(&result.value).unwrap()
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
        .wrap_err("get worker id failed")
        .unwrap();
    serde_json::from_value::<Vec<String>>(worker.remove(0)).unwrap()
}

pub async fn get_worker_list(mut client: &mut ConnectedClient) -> Vec<String> {
    let mut worker = utils::execute(
        &mut client,
        r#"
            (call relay ("worker" "list") [] workers)
        "#,
        "workers",
        Default::default(),
    )
        .await
        .wrap_err("get worker id failed")
        .unwrap();
    serde_json::from_value::<Vec<String>>(worker.remove(0)).unwrap()
}

pub async fn is_active(mut client: &mut ConnectedClient, deal: &str) -> eyre::Result<bool> {
    let mut is_active = utils::execute(
        &mut client,
        r#"
            (call relay ("worker" "is_active") [deal] result)"#,
        "result",
        hashmap! {"deal" => json!(deal) },
    )
        .await
        .wrap_err("is_active failed")?;
    serde_json::from_value::<bool>(is_active.remove(0)).wrap_err("parse is_active result")
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub aliases: Vec<String>,
    pub id: String,
    pub owner_id: String,
}

pub async fn service_list_on(
    mut client: &mut ConnectedClient,
    worker_id: &str,
) -> eyre::Result<Vec<Service>> {
    let mut result = utils::execute(
        &mut client,
        r#"
            (seq
                (call relay ("op" "noop") [])
                (call worker_id ("srv" "list") [] result)
            )
         "#,
        "result",
        hashmap! {"worker_id" => json!(worker_id) },
    )
        .await
        .wrap_err("srv.list failed")?;
    serde_json::from_value::<Vec<Service>>(result.remove(0)).wrap_err("parse is_active result")
}

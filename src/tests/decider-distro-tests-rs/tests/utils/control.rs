use crate::utils;
use crate::utils::spell;
use connected_client::ConnectedClient;
use eyre::WrapErr;
use fluence_spell_dtos::trigger_config::TriggerConfig;
use fluence_spell_dtos::value::ScriptValue;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub async fn update_config(
    client: &mut ConnectedClient,
    trigger_config: &TriggerConfig,
) -> eyre::Result<Vec<Value>> {
    utils::execute(
        client,
        r#"(call relay ("spell" "update_trigger_config") ["decider" config])"#,
        r#""done""#,
        hashmap! {
            "config" => json!(trigger_config),
        },
    )
    .await
}

// God left me here
pub fn modify_decider_spell_script(
    tmp_dir: PathBuf,
    decider_spell_id: String,
    updated_script: String,
) {
    let script_path: PathBuf = tmp_dir.join(
        [
            "services",
            "workdir",
            &decider_spell_id,
            "tmp",
            "script.air",
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    fs::write(&script_path, updated_script).unwrap();
}

pub async fn update_decider_script_for_tests(client: &mut ConnectedClient, test_dir: PathBuf) {
    let result = utils::execute(
        client,
        r#"
            (seq
                (call relay ("srv" "resolve_alias_opt") ["decider"] id)
                (call relay ("decider" "get_script_source_from_file") [] script)
            )
    "#,
        "id script",
        hashmap! {},
    )
    .await
    .unwrap();
    assert_eq!(
        result[0].as_array().unwrap().len(),
        1,
        "can't resolve `decider` alias"
    );
    let decider_id = result[0].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .to_string();
    let script = serde_json::from_value::<ScriptValue>(result[1].clone()).unwrap();
    assert!(script.success, "can't get decider script");

    let updated_script = format!(
        r#"
        (seq
            {script}
            (call "{client}" ("return" "") ["done"])
        )
    "#,
        client = client.peer_id,
        script = script.source_code,
    );

    modify_decider_spell_script(test_dir, decider_id, updated_script);
}

pub async fn wait_worker_spell_stopped(
    client: &mut ConnectedClient,
    worker_id: String,
    timeout_per_try: Duration,
) {
    let mut finished = false;
    for _ in 0..5 {
        // if only we can import these keys from Aqua files
        let strings = spell::list_get_strings_on(
            client,
            &worker_id,
            "worker-spell",
            "__installation_spell_status__",
        )
        .await
        .unwrap();
        assert!(
            strings.success,
            "can't get installation spell status: {}",
            strings.error
        );

        if !strings.strings.is_empty() {
            #[derive(Deserialize, Debug)]
            struct State {
                state: String,
            }
            let last_status = strings.strings.last().unwrap();
            println!("last status: {:?}", last_status);
            let state = serde_json::from_str::<State>(last_status).unwrap();
            let in_progress_statuses = ["INSTALLATION_IN_PROGRESS", "NOT_STARTED"];
            if !in_progress_statuses.contains(&state.state.as_str()) {
                assert_eq!(
                    state.state, "INSTALLATION_SUCCESSFUL",
                    "wrong installation spell status"
                );
                finished = true;
                break;
            }
        }
        tokio::time::sleep(timeout_per_try).await;
    }
    assert!(
        finished,
        "installation spell didn't finish in time or failed"
    );
}

pub async fn wait_decider_stopped(client: &mut ConnectedClient) {
    let decider_stopped = client
        .receive_args()
        .await
        .wrap_err("wait decider")
        .unwrap();

    assert_eq!(
        decider_stopped[0].as_str().unwrap(),
        "done",
        "decider ended with a different return status"
    );
}

use std::path::PathBuf;
use std::time::Duration;

use eyre::WrapErr;
use maplit::hashmap;
use serde::Deserialize;
use serde_json::{json, Value};

use connected_client::ConnectedClient;
use created_swarm::fluence_spell_dtos::trigger_config::TriggerConfig;
use created_swarm::fluence_spell_dtos::value::ScriptValue;

use crate::utils;
use crate::utils::chain::{play_chain, ChainReplies};
use crate::utils::test_rpc_server::ServerHandle;
use crate::utils::{oneshot_config, spell};

pub async fn run_decider(
    server: &mut ServerHandle,
    client: &mut ConnectedClient,
    chain_replies: ChainReplies,
) {
    update_decider_config(client, &oneshot_config())
        .await
        .unwrap();
    play_chain(server, chain_replies).await;
    wait_decider_stopped(client).await;
}

pub async fn update_decider_config(
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

pub async fn update_worker_config(
    client: &mut ConnectedClient,
    worker_id: &str,
    trigger_config: &TriggerConfig,
) -> eyre::Result<Vec<Value>> {
    utils::execute(
        client,
        r#"(seq
            (call relay ("op" "noop") [])
            (call worker_id ("spell" "update_trigger_config") ["worker-spell" config])
         )"#,
        r#""done""#,
        hashmap! {
            "worker_id" => json!(worker_id),
            "config" => json!(trigger_config),
        },
    )
    .await
}

// God left me here
pub async fn modify_decider_spell_script(
    persistent_base_dir: PathBuf,
    decider_spell_id: String,
    updated_script: String,
) {
    let script_path: PathBuf = persistent_base_dir.join(
        ["services", "workdir", &decider_spell_id, "script.air"]
            .iter()
            .collect::<PathBuf>(),
    );

    tokio::fs::write(&script_path, updated_script)
        .await
        .unwrap();
}

pub async fn update_decider_script_for_tests(
    client: &mut ConnectedClient,
    persistent_base_dir: PathBuf,
) {
    let result = utils::execute(
        client,
        r#"
            (seq
                (call relay ("srv" "resolve_alias_opt") ["decider"] id)
                (call relay ("decider" "get_script") [] script)
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
        script = script.value,
    );

    modify_decider_spell_script(persistent_base_dir, decider_id, updated_script).await;
}

pub async fn wait_worker_spell_stopped(
    client: &mut ConnectedClient,
    worker_id: &String,
    timeout_per_try: Duration,
) {
    wait_worker_spell_stopped_after(client, worker_id, 0, timeout_per_try).await;
}

pub async fn wait_worker_spell_stopped_after(
    client: &mut ConnectedClient,
    worker_id: &String,
    status_after: u64,
    timeout_per_try: Duration,
) {
    let mut finished = false;
    for _ in 0..10 {
        // if only we can import these keys from Aqua files
        let strings = spell::list_get_strings_on(
            client,
            worker_id,
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

        if !strings.value.is_empty() {
            #[derive(Deserialize, Debug)]
            struct State {
                state: String,
                timestamp: u64,
            }

            // HACK: sometimes sqlite returns trash in the requested lists.
            // FOR NOW we filter out the trash to avoid parsing errors and CI failures
            let last_statuses = strings
                .value
                .iter()
                .filter_map(|s| serde_json::from_str::<State>(s).ok())
                .filter(|s| s.timestamp >= status_after)
                .collect::<Vec<_>>();

            if let Some(state) = last_statuses.last() {
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

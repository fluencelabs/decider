/*
 * Nox Fluence Peer
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use eyre::WrapErr;
use maplit::hashmap;
use serde_json::json;

use connected_client::ConnectedClient;
use created_swarm::fluence_spell_dtos::value::{StringListValue, StringValue, U32Value, UnitValue};

use crate::utils::execute;

pub async fn get_string(
    mut client: &mut ConnectedClient,
    spell_id: &str,
    key: &str,
) -> eyre::Result<StringValue> {
    let relay = client.node.to_string();
    get_string_on(&mut client, &relay, spell_id, key).await
}

pub async fn get_string_on(
    mut client: &mut ConnectedClient,
    worker_id: &str,
    spell_id: &str,
    key: &str,
) -> eyre::Result<StringValue> {
    let result = execute(
        &mut client,
        r#"
       (seq
            (call relay ("op" "noop") [])
            (call worker_id (spell_id "get_string") [key] result)
       )
       "#,
        "result",
        hashmap! {
            "key" => json!(key),
            "worker_id" => json!(worker_id) ,
            "spell_id" => json!(spell_id)
        },
    )
    .await
    .wrap_err("get_string failed")?
    .pop()
    .ok_or(eyre::eyre!("no result of particle execution"))?;
    serde_json::from_value::<StringValue>(result).wrap_err("failed to parse StringValue")
}

pub async fn list_get_strings(
    mut client: &mut ConnectedClient,
    spell_id: &str,
    key: &str,
) -> eyre::Result<StringListValue> {
    let relay = client.node.to_string();
    list_get_strings_on(&mut client, &relay, spell_id, key).await
}

pub async fn list_get_strings_on(
    mut client: &mut ConnectedClient,
    worker_id: &str,
    spell_id: &str,
    key: &str,
) -> eyre::Result<StringListValue> {
    let result = execute(
        &mut client,
        r#"
       (seq
            (call relay ("op" "noop") [])
            (call worker_id (spell_id "list_get_strings") [key] result)
       )
       "#,
        "result",
        hashmap! {
            "key" => json!(key),
            "worker_id" => json!(worker_id),
            "spell_id" => json!(spell_id),
        },
    )
    .await
    .wrap_err("list_get_strings failed")?
    .pop()
    .ok_or(eyre::eyre!("no result of particle execution"))?;

    serde_json::from_value::<StringListValue>(result).wrap_err("failed to parse StringListValue")
}

pub async fn list_remove_string(
    mut client: &mut ConnectedClient,
    spell_id: &str,
    key: &str,
    value: &str,
) -> eyre::Result<UnitValue> {
    let relay = client.node.to_string();
    let result = execute(
        &mut client,
        r#"
       (seq
            (call relay ("op" "noop") [])
            (call worker_id (spell_id "list_remove_string") [key value] result)
       )
       "#,
        "result",
        hashmap! {
            "key" => json!(key),
            "value" => json!(value),
            "worker_id" => json!(relay),
            "spell_id" => json!(spell_id)
        },
    )
    .await
    .wrap_err("get_string failed")?
    .pop()
    .ok_or(eyre::eyre!("no result of particle execution"))?;

    serde_json::from_value::<UnitValue>(result).wrap_err("failed to parse StringListValue")
}

pub async fn get_u32_on(
    mut client: &mut ConnectedClient,
    worker_id: &str,
    spell_id: &str,
    key: &str,
) -> eyre::Result<U32Value> {
    let result = execute(
        &mut client,
        r#"
       (seq
            (call relay ("op" "noop") [])
            (call worker_id (spell_id "get_u32") [key] result)
       )
       "#,
        "result",
        hashmap! {
            "key" => json!(key),
            "worker_id" => json!(worker_id) ,
            "spell_id" => json!(spell_id)
        },
    )
    .await
    .wrap_err("get_string failed")?
    .pop()
    .ok_or(eyre::eyre!("no result of particle execution"))?;
    serde_json::from_value::<U32Value>(result).wrap_err("failed to parse U32Value")
}

pub async fn get_counter_on(
    client: &mut ConnectedClient,
    worker_id: &str,
    spell_id: &str,
) -> eyre::Result<u32> {
    let result = get_u32_on(client, worker_id, spell_id, "hw_counter").await?;
    assert!(result.success, "couldn't get hw_counter: {result:?}");
    assert!(!result.absent, "not hw_counter found: {result:?}");
    Ok(result.value)
}

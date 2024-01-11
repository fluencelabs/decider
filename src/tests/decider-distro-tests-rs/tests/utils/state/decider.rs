use crate::utils::spell;
use connected_client::ConnectedClient;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SyncInfo {
    pub blocks_diff: u32,
    pub run_updated: u32,
}

pub async fn get_sync_info(client: &mut ConnectedClient) -> eyre::Result<SyncInfo> {
    let result = spell::get_string(client, "decider", "sync_info")
        .await
        .wrap_err("get sync_info failed")?;
    if !result.success {
        return Err(eyre::eyre!("get sync_info failed: {}", result.error));
    }

    serde_json::from_str::<SyncInfo>(&result.value).wrap_err("parse sync_info")
}

pub async fn get_last_seen(client: &mut ConnectedClient) -> eyre::Result<Option<String>> {
    let result = spell::get_string(client, "decider", "last_seen_block")
        .await
        .wrap_err("get last_seen_block failed")?;
    if !result.success {
        return Err(eyre::eyre!("get sync_info failed: {}", result.error));
    }
    Ok(if result.absent {
        None
    } else {
        Some(result.value)
    })
}

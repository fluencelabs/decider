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

use serde::Deserialize;

use connected_client::ConnectedClient;

use crate::utils::spell;

#[derive(Deserialize, Debug)]
pub struct JoinedDeal {
    pub deal_id: String,
}

pub async fn get_joined_deals(client: &mut ConnectedClient) -> Vec<JoinedDeal> {
    let deals = spell::list_get_strings(client, "decider", "installed_deals")
        .await
        .unwrap();
    assert!(deals.success, "empty list of joined_deals: {}", deals.error);
    deals
        .value
        .into_iter()
        .map(|deal_id| JoinedDeal { deal_id })
        .collect()
}

pub async fn get_deal_tx_hash(
    client: &mut ConnectedClient,
    deal_id: &str,
) -> eyre::Result<Option<String>> {
    let key = format!("deal:tx_hash:{deal_id}");
    let tx = spell::get_string(client, "decider", &key).await?;
    assert!(tx.success, "couldn't get {key}: {tx:?}");
    Ok(if tx.absent { None } else { Some(tx.value) })
}

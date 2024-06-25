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
        let tx_hash = spell::get_string(&mut client, "decider", &format!("tx_hash:{deal_id}"))
            .await
            .unwrap();
        assert!(tx_hash.success, "can't get tx_hash:{deal_id}: {tx_hash:?}");
        txs.push(WorkerTxInfo {
            deal_id,
            tx_hash: tx_hash.value,
        })
    }
    txs
}

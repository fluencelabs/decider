use cid::Cid;
use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;
use serde::Deserialize;

use crate::chain::chain_data::EventField::{Indexed, NotIndexed};
use crate::chain::chain_data::{ChainData, ChainDataError, EventField};
use crate::chain::chain_event::ChainEvent;
use crate::chain::data_tokens::next_opt;
use crate::chain::u256::U256;
use crate::peer_id::parse_peer_id;

#[derive(Debug, Deserialize)]
#[marine]
pub struct DealPeerRemovedData {
    compute_peer_id: String,
    compute_unit_id: String,
}

#[derive(Debug)]
#[marine]
pub struct DealPeerRemoved {
    block_number: String,
    info: DealPeerRemovedData,
}

impl DealPeerRemoved {
    pub const EVENT_NAME: &'static str = "ComputeUnitRemoved";
}

impl ChainData for DealPeerRemovedData {
    fn event_name() -> &'static str {
        DealPeerRemoved::EVENT_NAME
    }

    fn signature() -> Vec<EventField> {
        vec![
            // compute peer id
            Indexed(ParamType::FixedBytes(32)),
            // compute unit id
            NotIndexed(ParamType::FixedBytes(32)),
        ]
    }

    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, ChainDataError> {
        let compute_peer_id =  next_opt(data_tokens, "compute_peer", Token::into_fixed_bytes)?;
        let compute_peer_id= parse_peer_id(compute_peer_id)?.to_string();

        let compute_unit_id = next_opt(data_tokens, "compute_unit_id", Token::into_fixed_bytes)?;
        let compute_unit_id = hex::encode(&compute_unit_id);

        Ok(DealPeerRemovedData { compute_peer_id, compute_unit_id })
    }
}

impl ChainEvent<DealPeerRemovedData> for DealPeerRemoved {
    fn new(block_number: String, info: DealPeerRemovedData) -> Self {
        Self { block_number, info }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::chain_data::ChainData;
    use crate::chain::log::{parse_log, Log};
    use crate::jsonrpc::JsonRpcResp;
    use serde_json::json;

    #[test]
    fn topic() {
        assert_eq!(
            DealPeerRemovedData::topic(),
            String::from("0x319861d91651b1d776940afecf33ec848245a2b1fb1854613bbe2a367b54f792")
        );
    }

    #[test]
    fn parse_peer_removed() {
        let jsonrpc = json!(
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": [
              {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0x304435e67fc799634ea6b659e1bb5b6c83eca598dc311327826a41d376149478",
                "blockHash": "0x8e5c5c30d83278a875493d6dcb02f2c8e737095a6f8ba46a1f00949a498b50c2",
                "blockNumber": "0x16c",
                "address": "0xeb92a1b5c10ad7bfdcaf23cb7dda9ea062cd07e8",
                "data": "0x9d0cd1f4517ced82e8e5ac8b9cb1bfffffc8dda1af6092b505d3be53f20550a0",
                "topics": [
                  "0x319861d91651b1d776940afecf33ec848245a2b1fb1854613bbe2a367b54f792"
                ]
              }
            ]
        });
        let data = "0x9d0cd1f4517ced82e8e5ac8b9cb1bfffffc8dda1af6092b505d3be53f20550a0".to_string();
        let log = Log {
            data,
            block_number: "0x16c".to_string(),
            removed: false,
            topics: vec![
                "0x319861d91651b1d776940afecf33ec848245a2b1fb1854613bbe2a367b54f792".to_string(),
            ],
        };
        let result = parse_log::<DealPeerRemovedData, DealPeerRemoved>(log);

        println!("{:?}", result);
    }
}

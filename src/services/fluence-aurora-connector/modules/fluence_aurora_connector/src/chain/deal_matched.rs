use cid::Cid;
use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;

use crate::chain::chain_data::EventField::{Indexed, NotIndexed};
use crate::chain::chain_data::{ChainData, DealParseError, EventField};
use crate::chain::chain_event::ChainEvent;
use crate::chain::data_tokens::next_opt;
use crate::chain::u256::U256;

/// Corresponding Solidity type:
/// ```solidity
/// struct CIDV1 {
///     bytes4 prefixes;
///     bytes32 hash;
/// }
///
/// event Matched(
///     address indexed computeProvider,
///     address deal,
///     uint joinedWorkers,
///     uint dealCreationBlock,
///     CIDV1 appCID
/// )
/// ```

#[derive(Debug, Clone)]
#[marine]
struct CIDV1 {
    prefixes: Vec<u8>,
    hash: Vec<u8>,
}

#[derive(Debug, Clone)]
#[marine]
pub struct Match {
    compute_provider: String,
    deal_id: String,
    joined_workers: U256,
    deal_creation_block: U256,
    app_cid: String,
}

#[derive(Debug, Clone)]
#[marine]
pub struct DealMatched {
    block_number: String,
    info: Match,
}

impl DealMatched {
    pub const EVENT_NAME: &'static str = "Matched";
}

impl ChainData for Match {
    fn event_name() -> &'static str {
        DealMatched::EVENT_NAME
    }

    fn signature() -> Vec<EventField> {
        vec![
            // compute_provider
            Indexed(ParamType::Address),
            // deal
            NotIndexed(ParamType::Address),
            // joined_workers
            NotIndexed(ParamType::Uint(256)),
            // deal_creation_block
            NotIndexed(ParamType::Uint(256)),
            // app_cid
            NotIndexed(ParamType::Tuple(vec![
                // prefixes
                ParamType::FixedBytes(4),
                // hash
                ParamType::FixedBytes(32),
            ])),
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, DealParseError> {
        let tokens = &mut data_tokens.into_iter();
        let compute_provider = next_opt(tokens, "compute_provider", Token::into_address)?;
        let deal = next_opt(tokens, "deal", Token::into_address)?;
        let joined_workers = next_opt(tokens, "joined_workers", U256::from_token)?;
        let deal_creation_block = next_opt(tokens, "deal_creation_block", U256::from_token)?;

        let app_cid = &mut next_opt(tokens, "app_cid", Token::into_tuple)?.into_iter();
        let cid_prefixes = next_opt(app_cid, "app_cid.prefixes", Token::into_fixed_bytes)?;
        let cid_hash = next_opt(app_cid, "app_cid.cid_hash", Token::into_fixed_bytes)?;
        let cid_bytes = [cid_prefixes, cid_hash].concat();
        let app_cid = Cid::read_bytes(cid_bytes.as_slice())?.to_string();

        Ok(Match {
            compute_provider: format!("{compute_provider:#x}"),
            deal_id: format!("{deal:#x}"),
            joined_workers,
            deal_creation_block,
            app_cid,
        })
    }
}

impl ChainEvent<Match> for DealMatched {
    fn new(block_number: String, info: Match) -> Self {
        Self { block_number, info }
    }
}

#[cfg(test)]
mod tests {
    use crate::chain::chain_data::ChainData;
    use crate::chain::deal_matched::{DealMatched, Match};
    use crate::chain::log::{parse_log, Log};
    use crate::jsonrpc::JsonRpcResp;

    #[test]
    fn topic() {
        assert_eq!(
            Match::topic(),
            String::from("0x8a2ecab128faa476aff507c7f34da3348b5c56e4a0401825f6919b4cc7b249f1")
        );
    }

    #[test]
    fn parse() {
        let jsonrpc = r#"
        {
            "jsonrpc": "2.0",
            "id": 0,
            "result": [
                {
                    "removed": false,
                    "logIndex": "0xb",
                    "transactionIndex": "0x0",
                    "transactionHash": "0x1a7122fa7501f09f19f29451548e88adf7ec88c99d34b4abdd09b27dfdbd74f1",
                    "blockHash": "0x1c6808f9f4f99bdad9a63601e07230b84effaec5aba724963ef17651131cf75d",
                    "blockNumber": "0x4e",
                    "address": "0x6328bb918a01603adc91eae689b848a9ecaef26d",
                    "data": "0x00000000000000000000000099e28f59ddfe14ff4e598a3ba3928bbf87b3f2b30000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000004d0155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e3",
                    "topics": [
                        "0x8a2ecab128faa476aff507c7f34da3348b5c56e4a0401825f6919b4cc7b249f1",
                        "0x0000000000000000000000006f10e8209296ea9e556f80b0ff545d8175f271d0"
                    ]
                }
            ]
        }
        "#;

        let logs: JsonRpcResp<Vec<Log>> = serde_json::from_str(jsonrpc).expect("invalid jsonrpc");
        let log = logs.result[0].clone();
        let m = parse_log::<Match, DealMatched>(log).expect("error parsing Match from log");
        assert_eq!(m.block_number, "0x4e");
        let m = m.info;
        assert_eq!(
            m.compute_provider,
            "6f10e8209296ea9e556f80b0ff545d8175f271d0"
        );
        assert_eq!(
            m.deal_id.to_lowercase(),
            "99e28f59ddfe14ff4e598a3ba3928bbf87b3f2b3"
        );
        assert_eq!(m.joined_workers.to_eth().as_u32(), 3);
        assert_eq!(m.deal_creation_block.to_eth().as_u32(), 77);
        assert_eq!(
            m.app_cid.to_string(),
            "bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m"
        );
    }
}

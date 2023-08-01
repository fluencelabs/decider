use cid::Cid;
use ethabi::param_type::ParamType;
use ethabi::Token;
use libp2p_identity::PeerId;
use marine_rs_sdk::marine;

use crate::chain::chain_data::EventField::{Indexed, NotIndexed};
use crate::chain::chain_data::{ChainData, EventField, LogParseError};
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
/// event ComputePeerMatched(
///     bytes32 indexed peerId
///     address deal
///     bytes32[] patIds
///     uint dealCreationBlock
///     CIDV1 appCID
/// );
/// ```

const PEER_ID_PREFIX: &[u8] = &[0, 36, 8, 1, 18, 32];

#[derive(Debug, Clone)]
#[marine]
pub struct Match {
    compute_peer: PeerId,
    deal_id: String,
    pat_ids: Vec<Vec<u8>>,
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
    pub const EVENT_NAME: &'static str = "ComputePeerMatched";
}

impl ChainData for Match {
    fn event_name() -> &'static str {
        DealMatched::EVENT_NAME
    }

    fn signature() -> Vec<EventField> {
        vec![
            // compute_provider
            Indexed(ParamType::FixedBytes(32)),
            // deal
            NotIndexed(ParamType::Address),
            // pat_ids
            NotIndexed(ParamType::Array(Box::new(ParamType::FixedBytes(32)))),
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
    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, LogParseError> {
        let tokens = &mut data_tokens.into_iter();

        let compute_peer = next_opt(tokens, "compute_peer", parse_peer_id)?;

        let deal = next_opt(tokens, "deal", Token::into_address)?;
        let pat_ids = next_opt(tokens, "pat_ids", |t| {
            Token::into_array(t)?
                .into_iter()
                .map(|t| t.into_fixed_bytes())
                .collect::<Option<Vec<_>>>()
        })?;
        let deal_creation_block = next_opt(tokens, "deal_creation_block", U256::from_token)?;

        let app_cid = &mut next_opt(tokens, "app_cid", Token::into_tuple)?.into_iter();
        let cid_prefixes = next_opt(app_cid, "app_cid.prefixes", Token::into_fixed_bytes)?;
        let cid_hash = next_opt(app_cid, "app_cid.cid_hash", Token::into_fixed_bytes)?;
        let cid_bytes = [cid_prefixes, cid_hash].concat();
        let app_cid = Cid::read_bytes(cid_bytes.as_slice())?.to_string();

        Ok(Match {
            compute_peer,
            deal_id: format!("{deal:#x}"),
            pat_ids,
            deal_creation_block,
            app_cid,
        })
    }
}

fn parse_peer_id(token: Token) -> Option<PeerId> {
    let bytes = Token::into_fixed_bytes(token)?;
    let peer_id = &[PEER_ID_PREFIX, &bytes].concat();

    PeerId::from_bytes(&peer_id).ok()
}

impl ChainEvent<Match> for DealMatched {
    fn new(block_number: String, info: Match) -> Self {
        Self { block_number, info }
    }
}

#[cfg(test)]
mod tests {
    use crate::chain::chain_data::ChainData;
    use crate::chain::deal_matched::{parse_peer_id, DealMatched, Match};
    use crate::chain::log::{parse_log, Log};
    use crate::jsonrpc::JsonRpcResp;
    use ethabi::Token;

    #[test]
    fn topic() {
        assert_eq!(
            Match::topic(),
            String::from("0x55e61a24ecdae954582245e5e611fb06905d6af967334fff4db72793bebc72a9")
        );
    }

    #[test]
    fn peer_id() {
        let bytes = [
            88, 198, 255, 218, 126, 170, 188, 84, 84, 39, 255, 137, 18, 55, 7, 139, 121, 207, 149,
            42, 196, 115, 102, 160, 4, 47, 227, 62, 7, 53, 189, 15,
        ];
        let peer_id =
            parse_peer_id(Token::FixedBytes(bytes.into())).expect("parse peer_id from Token");
        assert_eq!(
            peer_id.to_string(),
            String::from("12D3KooWFnv3Qc25eKpTDCNBoW1jXHMHHHSzcJoPkHai1b2dHNra")
        );

        let hex = "7a82a5feefcaad4a89c689412031e5f87c02b29e3fced583be5f05c7077354b7";
        let bytes = hex::decode(hex).expect("parse peer_id from hex");
        let bytes = Token::FixedBytes(bytes);
        let peer_id = parse_peer_id(bytes).expect("parse peer_id from Token");
        assert_eq!(
            peer_id.to_string(),
            String::from("12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE")
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
            m.compute_peer.to_string(),
            "6f10e8209296ea9e556f80b0ff545d8175f271d0"
        );
        assert_eq!(
            m.deal_id.to_lowercase(),
            "99e28f59ddfe14ff4e598a3ba3928bbf87b3f2b3"
        );
        assert_eq!(m.pat_ids.len(), 33);
        assert_eq!(m.deal_creation_block.to_eth().as_u32(), 77);
        assert_eq!(
            m.app_cid.to_string(),
            "bafkreifolrizgmusl4y7or5e5xmvr623a6i3ca4d5rwv457cezhschqj4m"
        );
    }
}

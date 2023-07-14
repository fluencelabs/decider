use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;

use crate::chain::chain_data::{ChainData, DealParseError};
use crate::chain::chain_event::ChainEvent;
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

#[derive(Debug)]
#[marine]
pub struct Match {
    compute_provider: String,
    deal: String,
    joined_workers: U256,
    deal_creation_block: U256,
    app_cid: CIDV1,
}

#[derive(Debug)]
#[marine]
pub struct DealMatched {
    block_number: String,
    info: Match,
}

impl DealMatched {
    pub const EVENT_NAME: &'static str = "Matched";
}

impl ChainData for Match {
    fn topic() -> String {
        let sig = Self::signature();
        let hash = ethabi::long_signature(DealMatched::EVENT_NAME, &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
    }

    fn signature() -> Vec<ParamType> {
        vec![
            // compute_provider
            ParamType::Address,
            // deal
            ParamType::Address,
            // joined_workers
            ParamType::Uint(256),
            // deal_creation_block
            ParamType::Uint(256),
            // app_cid
            ParamType::Tuple(vec![
                // prefixes
                ParamType::FixedBytes(4),
                // hash
                ParamType::FixedBytes(32),
            ]),
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data_tokens: Vec<Token>) -> Result<Self, DealParseError> {
        let deal_data: Option<Match> = try {
            let mut data_tokens = data_tokens.into_iter();
            let compute_provider = data_tokens.next()?.to_string();
            let deal = data_tokens.next()?.to_string();
            let joined_workers = U256::from_eth(data_tokens.next()?.into_uint()?);
            let deal_creation_block = U256::from_eth(data_tokens.next()?.into_uint()?);

            let mut app_cid = data_tokens.next()?.into_tuple()?.into_iter();
            let app_cid = CIDV1 {
                prefixes: app_cid.next()?.into_fixed_bytes()?,
                hash: app_cid.next()?.into_fixed_bytes()?,
            };

            Match {
                compute_provider,
                deal,
                joined_workers,
                deal_creation_block,
                app_cid,
            }
        };
        deal_data.ok_or_else(|| DealParseError::SignatureMismatch(Self::signature()))
    }
}

impl ChainEvent<Match> for DealMatched {
    fn new(block_number: String, info: Match) -> Self {
        Self { block_number, info }
    }
}

#[cfg(test)]
mod tests {
    use crate::chain::chain_data::{parse_chain_data, ChainData};
    use crate::chain::deal_matched::Match;
    use crate::chain::log::Log;
    use crate::jsonrpc::JsonRpcResp;
    use ethabi::ParamType;
    use ethabi::ParamType::Address;

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

        let log: JsonRpcResp<Vec<Log>> = serde_json::from_str(jsonrpc).expect("invalid jsonrpc");
        let data = &log.result[0].data;
        let signature = Match::signature();

        {
            let data = hex::decode("00000000000000000000000099e28f59ddfe14ff4e598a3ba3928bbf87b3f2b30000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000004d0155122000000000000000000000000000000000000000000000000000000000ae5c519332925f31f747a4edd958fb5b0791b10383ec6d5e77e2264f211e09e3").expect("invalid hex in data");
            let address = ethabi::decode(&[Address], &data[0..32]).expect("ethabi address fail");
            println!("address {:?}", address);
            let address = ethabi::decode(&[Address], &data[32..64]).expect("ethabi address fail");
            println!("address {:?}", address);
            // ethabi::decode(&signature, &data).expect("ethabi error");
        }

        // let tokens = parse_chain_data(data, signature);
        // println!("tokens {:?}", tokens);
        // let tokens = tokens.unwrap();
        // let m = Match::parse(tokens);
        // println!("match {:?}", m)
    }
}

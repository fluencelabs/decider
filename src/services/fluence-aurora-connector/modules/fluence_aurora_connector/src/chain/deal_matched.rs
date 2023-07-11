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

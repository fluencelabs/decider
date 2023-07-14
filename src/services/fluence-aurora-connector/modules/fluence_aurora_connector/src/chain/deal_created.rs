use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;

use crate::chain::chain_data::EventField::NotIndexed;
use crate::chain::chain_data::{ChainData, DealParseError, EventField};
use crate::chain::chain_event::ChainEvent;
use crate::chain::data_tokens::next_opt;
use crate::chain::u256::U256;

/// Corresponding Solidity type:
/// ```solidity
///event DealCreated(
///    address deal,
///    address paymentToken,
///    uint256 pricePerEpoch,
///    uint256 requiredStake,
///    uint256 minWorkers,
///    uint256 maxWorkersPerProvider,
///    uint256 targetWorkers,
///    string appCID,
///    string[] effectorWasmsCids,
///    uint256 epoch
///);
/// ```
#[derive(Debug)]
#[marine]
pub struct DealCreatedData {
    /// Address of newly created deal contract
    deal_id: String,
    /// Token used to pay for the deal
    payment_token: String,
    /// How much to pay per epoch
    price_per_epoch: U256,
    /// How much a peer should pay to join the deal
    required_stake: U256,
    /// Minimum required workers
    min_workers: u64,
    /// Maximum required workers
    max_workers_per_provider: u64,
    /// Desired amount of workers
    target_workers: u64,
    /// Target application CID
    app_cid: String,
    /// CIDs of required effectors
    effector_wasms_cids: Vec<String>,
    /// Number of epoch
    epoch: u64,
}

#[derive(Debug)]
#[marine]
pub struct DealCreated {
    block_number: String,
    info: DealCreatedData,
}

impl DealCreated {
    pub const EVENT_NAME: &str = "DealCreated";
}

impl ChainData for DealCreatedData {
    fn event_name() -> &'static str {
        DealCreated::EVENT_NAME
    }

    fn signature() -> Vec<EventField> {
        vec![
            NotIndexed(ParamType::Address),                            // deal
            NotIndexed(ParamType::Address),                            // paymentToken
            NotIndexed(ParamType::Uint(256)),                          // pricePerEpoch
            NotIndexed(ParamType::Uint(256)),                          // requiredStake
            NotIndexed(ParamType::Uint(256)),                          // minWorkers
            NotIndexed(ParamType::Uint(256)),                          // maxWorkersPerProvider
            NotIndexed(ParamType::Uint(256)),                          // targetWorkers
            NotIndexed(ParamType::String),                             // appCID
            NotIndexed(ParamType::Array(Box::new(ParamType::String))), // effectorWasmsCids
            NotIndexed(ParamType::Uint(256)),                          // epoch
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, DealParseError> {
        let deal_id = next_opt(data_tokens, "deal_id", Token::into_string)?;
        let payment_token = next_opt(data_tokens, "payment_token", Token::into_string)?;

        let price_per_epoch = next_opt(data_tokens, "price_per_epoch", U256::from_token)?;
        let required_stake = next_opt(data_tokens, "required_stake", U256::from_token)?;

        let min_workers = next_opt(data_tokens, "min_workers", Token::into_uint)?.as_u64();
        let max_workers_per_provider =
            next_opt(data_tokens, "max_workers_per_provider", Token::into_uint)?.as_u64();
        let target_workers = next_opt(data_tokens, "target_workers", Token::into_uint)?.as_u64();

        let app_cid = next_opt(data_tokens, "app_cid", Token::into_string)?;
        let effector_wasms_cids = next_opt(data_tokens, "effector_wasms_cids", |t| {
            t.into_array()?
                .into_iter()
                .map(Token::into_string)
                .collect()
        })?;
        let epoch = next_opt(data_tokens, "epoch", Token::into_uint)?.as_u64();

        Ok(DealCreatedData {
            deal_id,
            payment_token,
            price_per_epoch,
            required_stake,
            min_workers,
            max_workers_per_provider,
            target_workers,
            app_cid,
            effector_wasms_cids,
            epoch,
        })
    }
}

impl ChainEvent<DealCreatedData> for DealCreated {
    fn new(block_number: String, info: DealCreatedData) -> Self {
        Self { block_number, info }
    }
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use ethabi::Token;

    use crate::chain::chain_data::ChainData;
    use crate::chain::chain_data::DealParseError;
    use crate::chain::deal_created::{DealCreated, DealCreatedData};
    use crate::chain::log::{parse_log, Log};

    // Cannot now provide an example of encoded data with effectors
    // #[test]
    // fn test_chain_parsing_ok() {
    //     let result = parse_chain_deal_data(data);
    //     assert!(result.is_ok(), "can't parse data: {:?}", result);
    //     let result = result.unwrap();
    //     assert_eq!(result.deal_id, "7b7b3258e5a3b82f8e67f1a3bff149b4c8534b7c");
    //     assert_eq!(result.payment_token, "732bfdbb03de27c5a5915f5ccdee85080d1d4c3d");
    //     assert_eq!(result.required_stake.to_eth(), ethabi::ethereum_types::U256::exp10(18));
    //     assert_eq!(result.price_per_epoch.to_eth(), ethabi::ethereum_types::U256::exp10(18));
    //     assert_eq!(result.min_workers, 3);
    //     assert_eq!(result.max_workers_per_provider, 8);
    //     assert_eq!(result.target_workers, 5);
    //     assert_eq!(result.app_cid, "pafkreihszin3nr7ja7ig3l7enb7fph6oo2zx4tutw5qfaiw2kltmzqtp2i");
    //     assert_eq!(result.effector_wasms_cids, vec!["bafkreihszin3nr7ja7ig3l7enb7fph6oo2zx4tutw5qfaiw2kltmzqtp2i"]);
    //     assert_eq!(result.epoch, 0);
    // }

    #[test]
    fn test_chain_parsing_ok_empty_effectors() {
        let data = "0x00000000000000000000000094952482aa36dc9ec113bbba0df49284ecc071e20000000000000000000000005f7a3a2dab601ee4a1970b53088bebca176e13f40000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000554509000000000000000000000000000000000000000000000000000000000000002e516d5758616131534b41445274774e7472773278714a5556447864734472536d4a635542614a7946324c353476500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
        let log = Log {
            data,
            block_number: "0x0".to_string(),
            removed: false,
            topics: vec![],
        };
        let result = parse_log::<DealCreatedData, DealCreated>(log);

        assert!(result.is_ok(), "can't parse data: {:?}", result);
        let result = result.unwrap().info;
        assert_eq!(result.deal_id, "94952482aa36dc9ec113bbba0df49284ecc071e2");
        assert_eq!(
            result.payment_token,
            "5f7a3a2dab601ee4a1970b53088bebca176e13f4"
        );
        assert_eq!(
            result.required_stake.to_eth(),
            ethabi::ethereum_types::U256::exp10(18)
        );
        assert_eq!(
            result.price_per_epoch.to_eth(),
            ethabi::ethereum_types::U256::exp10(18)
        );
        assert_eq!(result.min_workers, 3);
        assert_eq!(result.max_workers_per_provider, 10000000);
        assert_eq!(result.target_workers, 5);
        assert_eq!(
            result.app_cid,
            "QmWXaa1SKADRtwNtrw2xqJUVDxdsDrSmJcUBaJyF2L54vP"
        );
        let empty: Vec<String> = vec![];
        assert_eq!(result.effector_wasms_cids, empty);
        assert_eq!(result.epoch, 5588233);
    }

    #[test]
    fn test_chain_parsing_fail_empty() {
        let result = DealCreatedData::parse(&mut std::iter::empty());
        assert!(result.is_err());
        assert_matches!(result, Err(DealParseError::Empty));
    }

    #[test]
    fn test_chain_parsing_fail_something() {
        let data = &mut vec![Token::Bool(false)].into_iter();
        let result = DealCreatedData::parse(data);
        assert!(result.is_err());
        assert_matches!(
            result,
            Err(DealParseError::EthError(ethabi::Error::InvalidData))
        );
    }
}

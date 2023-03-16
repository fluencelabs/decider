use ethabi::param_type::ParamType;

use super::*;

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

#[marine]
pub struct DealCreated {
    block_number: String,
    /// The number of the block next to the one of the deal
    next_block_number: String,
    info: DealCreatedData,
}

impl DealCreated {
    pub const EVENT_NAME: &str = "DealCreated";
}

impl ChainData for DealCreatedData {
    fn topic() -> String {
        let sig = Self::signature();
        let hash = ethabi::long_signature(DealCreated::EVENT_NAME, &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
    }

    fn signature() -> Vec<ParamType> {
        vec![
            ParamType::Address,                            // deal
            ParamType::Address,                            // paymentToken
            ParamType::Uint(256),                          // pricePerEpoch
            ParamType::Uint(256),                          // requiredStake
            ParamType::Uint(256),                          // minWorkers
            ParamType::Uint(256),                          // maxWorkersPerProvider
            ParamType::Uint(256),                          // targetWorkers
            ParamType::String,                             // appCID
            ParamType::Array(Box::new(ParamType::String)), // effectorWasmsCids
            ParamType::Uint(256),                          // epoch
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data: &str) -> Result<DealCreatedData, DealParseError> {
        let data_tokens = parse_chain_data(data, Self::signature())?;
        let deal_data: Option<DealCreatedData> = try {
            let deal_id = data_tokens[0].to_string();
            let payment_token = data_tokens[1].to_string();

            let price_per_epoch = U256::from_eth(data_tokens[2].clone().into_uint()?);
            let required_stake = U256::from_eth(data_tokens[3].clone().into_uint()?);

            let min_workers = data_tokens[4].clone().into_uint()?.as_u64();
            let max_workers_per_provider = data_tokens[5].clone().into_uint()?.as_u64();
            let target_workers = data_tokens[6].clone().into_uint()?.as_u64();

            let app_cid = data_tokens[7].clone().into_string()?;
            let effector_wasms_cids = data_tokens[8]
                .clone()
                .into_array()?
                .into_iter()
                .map(|x| x.into_string())
                .collect::<Option<_>>()?;
            let epoch = data_tokens[9].clone().into_uint()?.as_u64();

            DealCreatedData {
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
            }
        };
        deal_data.ok_or_else(|| {
            DealParseError::InternalError("parsed data doesn't correspond expected signature")
        })
    }
}

impl ChainEvent<DealCreatedData> for DealCreated {
    fn new(next_block_number: String, block_number: String, info: DealCreatedData) -> Self {
        Self { next_block_number, block_number, info }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_chain_deal_created_data, DealParseError};
    use std::assert_matches::assert_matches;

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
        let data = "0x00000000000000000000000094952482aa36dc9ec113bbba0df49284ecc071e20000000000000000000000005f7a3a2dab601ee4a1970b53088bebca176e13f40000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000554509000000000000000000000000000000000000000000000000000000000000002e516d5758616131534b41445274774e7472773278714a5556447864734472536d4a635542614a7946324c353476500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

        let result = parse_chain_deal_data(data);
        assert!(result.is_ok(), "can't parse data: {:?}", result);
        let result = result.unwrap();
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
        let data = "";
        let result = parse_chain_deal_data(data);
        assert!(result.is_err());
        assert_matches!(result, Err(DealParseError::Empty));
    }

    #[test]
    fn test_chain_parsing_fail_something() {
        let data = "0x1234567890";
        let result = parse_chain_deal_data(data);
        assert!(result.is_err());
        assert_matches!(
            result,
            Err(DealParseError::EthError(ethabi::Error::InvalidData))
        );
    }
}

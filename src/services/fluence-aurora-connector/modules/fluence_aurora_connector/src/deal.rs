use ethabi::decode;
use ethabi::param_type::ParamType;
use marine_rs_sdk::marine;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DealParseError {
    #[error(transparent)]
    EthError(#[from] ethabi::Error),
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
    #[error("internal error, please, contact developers: {0}")]
    InternalError(&'static str),
    #[error("empty data, nothing to parse")]
    Empty,
}

#[marine]
pub struct DealCreated {
    block_number: String,
    info: DealData,
}

impl DealCreated {
    pub fn new(block_number: String, info: DealData) -> Self {
        Self { block_number, info }
    }
}

#[derive(Debug, PartialEq)]
#[marine]
pub struct U256 {
    bytes: Vec<u8>,
}

impl U256 {
    pub fn from_bytes(bs: &[u8; 32]) -> Self {
        U256 { bytes: bs.to_vec() }
    }

    pub fn to_eth(&self) -> ethabi::ethereum_types::U256 {
        ethabi::ethereum_types::U256::from_little_endian(&self.bytes)
    }
}

/// Corresponding Solidity type:
/// ```solidity
/// event DealCreated(
///         address deal,
///         address paymentToken,
///         uint256 pricePerEpoch,
///         uint256 requiredStake,
///         uint256 minWorkers,
///         uint256 maxWorkers,
///         uint256 targetWorkers,
///         bytes32 appCID,
///         bytes32[] effectorWasmsCids
///     );
/// ```
#[derive(Debug)]
#[marine]
pub struct DealData {
    /// Address of newly created deal contract
    deal_id: String,
    /// Token used to pay for the deal
    payment_token: String,
    ///
    price_per_epoch: U256,
    /// How much a peer should pay to join the deal
    required_stake: U256,
    /// Minimum required workers
    min_workers: u32,
    /// Maximum required workers
    max_workers: u32,
    /// Desired amount of workers
    target_workers: u32,
    /// Target application CID
    app_cid: String,
    /// CIDs of required effectors
    effector_wasms_cids: Vec<String>,
}

/// Parse data from chain. Accepts data with and without "0x" prefix.
pub fn parse_chain_deal_data(data: &str) -> Result<DealData, DealParseError> {
    let data = data.strip_prefix("0x").unwrap_or(data);
    if data.len() == 0 {
        return Err(DealParseError::Empty);
    }
    let data = hex::decode(data)?;
    let types = vec![
        ParamType::Address,
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::String,
        ParamType::Array(Box::new(ParamType::String)),
    ];
    let result = decode(&types, &data)?;

    let deal_data: Option<DealData> = try {
        let deal_id = result[0].to_string();
        let payment_token = result[1].to_string();

        let price_per_epoch = convert_to_bytes(result[2].clone().into_uint()?);
        let required_stake = convert_to_bytes(result[3].clone().into_uint()?);

        let min_workers = result[4].clone().into_uint()?.as_u32();
        let max_workers = result[5].clone().into_uint()?.as_u32();
        let target_workers = result[6].clone().into_uint()?.as_u32();

        let app_cid = result[7].clone().into_string()?;
        let effector_wasms_cids = result[8]
            .clone()
            .into_array()?
            .into_iter()
            .map(|x| x.into_string())
            .collect::<Option<_>>()?;

        DealData {
            deal_id,
            payment_token,
            price_per_epoch,
            required_stake,
            min_workers,
            max_workers,
            target_workers,
            app_cid,
            effector_wasms_cids,
        }
    };
    deal_data.ok_or_else(|| {
        DealParseError::InternalError("parsed data doesn't correspond expected signature")
    })
}

fn convert_to_bytes(num: ethabi::ethereum_types::U256) -> U256 {
    let bytes = num
        .0
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<_>>();
    U256 { bytes }
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;
    use crate::{parse_chain_deal_data, DealParseError};

    #[test]
    fn test_chain_parsing_ok() {
        let data = "0x0000000000000000000000007b7b3258e5a3b82f8e67f1a3bff149b4c8534b7c000000000000000000000000732bfdbb03de27c5a5915f5ccdee85080d1d4c3d0000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000500000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000180000000000000000000000000000000000000000000000000000000000000003b7061666b72656968737a696e336e72376a61376967336c37656e6237667068366f6f327a783474757477357166616977326b6c746d7a7174703269000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003b6261666b72656968737a696e336e72376a61376967336c37656e6237667068366f6f327a783474757477357166616977326b6c746d7a71747032690000000000";

        let result = parse_chain_deal_data(data);
        assert!(result.is_ok(), "can't parse data: {:?}", result);
        let result = result.unwrap();
        assert_eq!(result.deal_id, "7b7b3258e5a3b82f8e67f1a3bff149b4c8534b7c");
        assert_eq!(result.payment_token, "732bfdbb03de27c5a5915f5ccdee85080d1d4c3d");
        assert_eq!(result.required_stake.to_eth(), ethabi::ethereum_types::U256::exp10(18));
        assert_eq!(result.price_per_epoch.to_eth(), ethabi::ethereum_types::U256::exp10(18));
        assert_eq!(result.min_workers, 3);
        assert_eq!(result.max_workers, 8);
        assert_eq!(result.target_workers, 5);
        assert_eq!(result.app_cid, "pafkreihszin3nr7ja7ig3l7enb7fph6oo2zx4tutw5qfaiw2kltmzqtp2i");
        assert_eq!(result.effector_wasms_cids, vec!["bafkreihszin3nr7ja7ig3l7enb7fph6oo2zx4tutw5qfaiw2kltmzqtp2i"]);
    }

    fn test_chain_parsing_ok_empty_effectors() {
        let data = "0x000000000000000000000000d41a9ee60c383de3f111d68a4c4bc627c23d638e000000000000000000000000732bfdbb03de27c5a5915f5ccdee85080d1d4c3d0000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000500000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000180000000000000000000000000000000000000000000000000000000000000003b7061666b72656968737a696e336e72376a61376967336c37656e6237667068366f6f327a783474757477357166616977326b6c746d7a717470326900000000000000000000000000000000000000000000000000000000000000000000000000";

        let result = parse_chain_deal_data(data);
        assert!(result.is_ok(), "can't parse data: {:?}", result);
        let result = result.unwrap();
        assert_eq!(result.deal_id, "7b7b3258e5a3b82f8e67f1a3bff149b4c8534b7c");
        assert_eq!(result.payment_token, "732bfdbb03de27c5a5915f5ccdee85080d1d4c3d");
        assert_eq!(result.required_stake.to_eth(), ethabi::ethereum_types::U256::exp10(18));
        assert_eq!(result.price_per_epoch.to_eth(), ethabi::ethereum_types::U256::exp10(18));
        assert_eq!(result.min_workers, 3);
        assert_eq!(result.max_workers, 8);
        assert_eq!(result.target_workers, 5);
        assert_eq!(result.app_cid, "pafkreihszin3nr7ja7ig3l7enb7fph6oo2zx4tutw5qfaiw2kltmzqtp2i");
        let empty: Vec<String> = vec![];
        assert_eq!(result.effector_wasms_cids, empty);
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
        assert_matches!(result, Err(DealParseError::EthError(ethabi::Error::InvalidData)));
    }
}

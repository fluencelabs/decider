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
    pub const EVENT_NAME: &str = "DealCreated";

    pub fn new(block_number: String, info: DealData) -> Self {
        Self { block_number, info }
    }

    pub fn topic() -> String {
        let sig = DealData::signature();
        let hash = ethabi::long_signature(Self::EVENT_NAME, &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
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
pub struct DealData {
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

impl DealData {
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
}

/// Parse data from chain. Accepts data with and without "0x" prefix.
pub fn parse_chain_deal_data(data: &str) -> Result<DealData, DealParseError> {
    let data = data.strip_prefix("0x").unwrap_or(data);
    if data.is_empty() {
        return Err(DealParseError::Empty);
    }
    let data = hex::decode(data)?;
    let types = DealData::signature();
    let result = ethabi::decode(&types, &data)?;

    let deal_data: Option<DealData> = try {
        let deal_id = result[0].to_string();
        let payment_token = result[1].to_string();

        let price_per_epoch = convert_to_bytes(result[2].clone().into_uint()?);
        let required_stake = convert_to_bytes(result[3].clone().into_uint()?);

        let min_workers = result[4].clone().into_uint()?.as_u64();
        let max_workers_per_provider = result[5].clone().into_uint()?.as_u64();
        let target_workers = result[6].clone().into_uint()?.as_u64();

        let app_cid = result[7].clone().into_string()?;
        let effector_wasms_cids = result[8]
            .clone()
            .into_array()?
            .into_iter()
            .map(|x| x.into_string())
            .collect::<Option<_>>()?;
        let epoch = result[9].clone().into_uint()?.as_u64();

        DealData {
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
    use crate::{parse_chain_deal_data, DealParseError};
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

use super::*;
use ethabi::param_type::ParamType;
use marine_rs_sdk::marine;

/// Corresponding Solidity type:
/// ```solidity
/// event NewAppCID(string appCID);
/// ```
#[derive(Debug)]
#[marine]
pub struct DealChangedData {
    ///
    app_cid: String,
}

#[marine]
pub struct DealChanged {
    block_number: String,
    info: DealChangedData,
}

impl DealChanged {
    pub const EVENT_NAME: &str = "NewAppCID";
}

impl ChainData for DealChangedData {
    fn topic() -> String {
        let sig = Self::signature();
        let hash = ethabi::long_signature(DealChanged::EVENT_NAME, &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
    }

    fn signature() -> Vec<ParamType> {
        vec![
            ParamType::String, // appCID
        ]
    }

    /// Parse data from chain. Accepts data with and without "0x" prefix.
    fn parse(data: &str) -> Result<DealChangedData, DealParseError> {
        let data_tokens = parse_chain_data(data, Self::signature())?;
        let deal_data: Option<DealChangedData> = try {
            let app_cid = data_tokens[0].clone().into_string()?;
            DealChangedData { app_cid }
        };
        deal_data.ok_or_else(|| {
            DealParseError::InternalError("parsed data doesn't correspond expected signature")
        })
    }
}

impl ChainEvent<DealChangedData> for DealChanged {
    fn new(block_number: String, info: DealChangedData) -> Self {
        Self { block_number, info }
    }
}

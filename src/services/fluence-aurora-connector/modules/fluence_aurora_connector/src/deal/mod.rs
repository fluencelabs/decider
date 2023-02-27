pub mod changed_cid;
pub mod created;

use ethabi::param_type::ParamType;
use ethabi::Token;
use marine_rs_sdk::marine;
use thiserror::Error;

pub trait ChainEvent<ChainData> {
    fn new(block_number: String, data: ChainData) -> Self;
}

pub trait ChainData {
    fn signature() -> Vec<ParamType>;
    fn parse(data: &str) -> Result<Self, DealParseError>
    where
        Self: Sized;
    fn topic() -> String;
}

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

/// Parse data from chain. Accepts data with and without "0x" prefix.
pub(crate) fn parse_chain_data(
    data: &str,
    signature: Vec<ParamType>,
) -> Result<Vec<Token>, DealParseError> {
    let data = data.strip_prefix("0x").unwrap_or(data);
    if data.is_empty() {
        return Err(DealParseError::Empty);
    }
    let data = hex::decode(data)?;
    Ok(ethabi::decode(&signature, &data)?)
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

    pub fn from_eth(num: ethabi::ethereum_types::U256) -> U256 {
        let bytes = num
            .0
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<_>>();
        U256 { bytes }
    }
}

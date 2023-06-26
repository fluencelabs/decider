use ethabi::{ParamType, Token};
use thiserror::Error;

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

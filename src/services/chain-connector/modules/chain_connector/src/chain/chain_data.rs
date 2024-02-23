use crate::chain::chain_data::ChainDataError::DecodeHex;
use ethabi::{ParamType, Token};
use hex::FromHexError;
use libp2p_identity::ParseError;
use thiserror::Error;

use crate::hex::decode_hex;

#[derive(Debug, Error)]
pub enum ChainDataError {
    #[error("empty data, nothing to parse")]
    Empty,
    #[error("missing token for field '{0}'")]
    MissingParsedToken(&'static str),
    #[error("invalid token for field '{0}'")]
    InvalidParsedToken(&'static str),
    #[error("invalid compute peer id: '{0}'")]
    InvalidComputePeerId(#[from] ParseError),
    #[error("data is not a valid hex: '{0}'")]
    DecodeHex(#[source] FromHexError),
    #[error(transparent)]
    EthError(#[from] ethabi::Error),
    #[error("invalid app_cid: {0:?}")]
    InvalidCID(#[from] cid::Error),
}

#[derive(Debug, Clone, PartialEq)]
/// Kind of the field in Chain Event
pub enum EventField {
    /// If field is indexed, it's passed among topics
    Indexed(ParamType),
    /// If field is not indexed, it's passed in log.data
    NotIndexed(ParamType),
}

impl EventField {
    pub fn param_type(self) -> ParamType {
        match self {
            EventField::Indexed(t) => t,
            EventField::NotIndexed(t) => t,
        }
    }
}

pub trait ChainData {
    fn event_name() -> &'static str;
    fn signature() -> Vec<EventField>;
    fn parse(data_tokens: &mut impl Iterator<Item = Token>) -> Result<Self, ChainDataError>
    where
        Self: Sized;

    fn topic() -> String {
        let sig: Vec<_> = Self::signature()
            .into_iter()
            .map(|t| t.param_type())
            .collect();
        let hash = ethabi::long_signature(Self::event_name(), &sig);
        format!("0x{}", hex::encode(hash.as_bytes()))
    }
}

/// Parse data from chain. Accepts data with and without "0x" prefix.
pub fn parse_chain_data(data: &str, signature: &[ParamType]) -> Result<Vec<Token>, ChainDataError> {
    if data.is_empty() {
        return Err(ChainDataError::Empty);
    }
    let data = decode_hex(data).map_err(DecodeHex)?;
    Ok(ethabi::decode(signature, &data)?)
}

use ethabi::{ParamType, Token};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum EventField {
    Indexed(ParamType),
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
    fn parse(data_tokens: Vec<Token>) -> Result<Self, DealParseError>
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

#[derive(Debug, Error)]
pub enum DealParseError {
    #[error(transparent)]
    EthError(#[from] ethabi::Error),
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
    #[error("parsed data doesn't correspond to the expected signature: {0:?}")]
    SignatureMismatch(Vec<EventField>),
    #[error("incorrect signature: not found token for field #{position} of type ${event_field:?}")]
    MissingToken {
        position: usize,
        event_field: EventField,
    },
    #[error("incorrect signature: not found topic for indexed field #{position} of type ${event_field:?}")]
    MissingTopic {
        position: usize,
        event_field: EventField,
    },
    #[error("empty data, nothing to parse")]
    Empty,
    #[error("invalid app_cid: {0:?}")]
    InvalidCID(#[from] cid::Error),
    #[error("missing token for field '{0}'")]
    MissingParsedToken(&'static str),
    #[error("invalid token for field '{0}'")]
    InvalidParsedToken(&'static str),
}

/// Parse data from chain. Accepts data with and without "0x" prefix.
pub(crate) fn parse_chain_data(
    data: &str,
    signature: &[ParamType],
) -> Result<Vec<Token>, DealParseError> {
    let data = data.strip_prefix("0x").unwrap_or(data);
    if data.is_empty() {
        return Err(DealParseError::Empty);
    }
    let data = hex::decode(data)?;
    Ok(ethabi::decode(signature, &data)?)
}

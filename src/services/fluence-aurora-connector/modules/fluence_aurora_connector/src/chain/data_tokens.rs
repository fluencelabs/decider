use ethabi::Token;

use crate::chain::chain_data::DealParseError;
use crate::chain::chain_data::DealParseError::{InvalidParsedToken, MissingParsedToken};

// Take next token and parse it with `f`
pub fn next_opt<T>(
    data_tokens: &mut impl Iterator<Item = Token>,
    name: &'static str,
    f: impl Fn(Token) -> Option<T>,
) -> Result<T, DealParseError> {
    let next = data_tokens.next().ok_or(MissingParsedToken(name))?;
    let parsed = f(next).ok_or(InvalidParsedToken(name))?;

    Ok(parsed)
}

// Take next token and parse it with `f`
pub fn next<T>(
    data_tokens: &mut impl Iterator<Item = Token>,
    name: &'static str,
    f: impl Fn(Token) -> T,
) -> Result<T, DealParseError> {
    next_opt(data_tokens, name, |t| Some(f(t)))
}

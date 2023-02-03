use marine_rs_sdk::marine;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DealParseError {
    #[error("expected encoded data of lentgh {expected} but got {actual}")]
    WrongLength { expected: usize, actual: usize },
}

#[marine]
pub struct Deal {
    block_number: String,
    info: DealData,
}

impl Deal {
    pub fn new(block_number: String, info: DealData) -> Self {
        Deal { block_number, info }
    }
}

#[derive(Debug)]
#[marine]
pub struct DealData {
    deal_id: String,
    payment_token: String,
    price_per_epoch: String,
    required_stake: String,
}

/* Data structure right now:
 * 1. address: 32 bytes must be truncated to 20
 * 2. paymentToken: 32 bytes must be truncated to 20
 * 3. pricePerEpoch: 32 bytes
 * 4. requiredStake: 32 bytes
 *
 * `data` is in a hex format
 */
pub fn parse_chain_deal_data(data: String) -> Result<DealData, DealParseError> {
    // Each field is 32 bytes, one byte is represented by two hex symbols.
    let field_len = 32 * 2;

    // field_len * number_of_field + len("0x")
    let expected_total_len = field_len * 4 + 2;
    if data.len() != expected_total_len {
        return Err(DealParseError::WrongLength {
            expected: expected_total_len,
            actual: data.len(),
        });
    }

    // truncate leading "0x"
    let (_, rest) = data.split_at(2);
    let (deal_id_raw, rest) = rest.split_at(field_len);
    let (payment_token_raw, rest) = rest.split_at(field_len);
    let (price_per_epoch, required_stake) = rest.split_at(field_len);

    // Real length of `deal_id` and `payment_token` is 20 bytes.
    let truncate_len = field_len - 40;
    let (_, deal_id) = deal_id_raw.split_at(truncate_len);
    let (_, payment_token) = payment_token_raw.split_at(truncate_len);

    Ok(DealData {
        deal_id: format!("0x{}", deal_id),
        payment_token: format!("0x{}", payment_token),
        price_per_epoch: format!("0x{}", price_per_epoch),
        required_stake: format!("0x{}", required_stake),
    })
}

#[cfg(test)]
mod test {
    use crate::{parse_chain_deal_data, DealParseError};
    use std::assert_matches::assert_matches;

    #[test]
    fn test_ok() {
        let deal_id_raw = "82c3a4aecc9ba6a503401f5f5fb746337ddf74d9";
        let deal_id_expected = format!("0x{}", deal_id_raw);

        let payment_token_raw = "9dba2bc689d786583e532e505b5187d85169509f";
        let payment_token_expected = format!("0x{}", payment_token_raw);

        let price_per_epoch_raw =
            "0000000000000000000000000000000000000000000000000de0b6b3a7640000";
        let price_per_epoch_expected = format!("0x{}", price_per_epoch_raw);

        let required_stake_raw = "0000000000000000000000000000000000000000000000000de0b6b3a7640000";
        let required_stake_expected = format!("0x{}", required_stake_raw);

        let data = format!(
            "0x000000000000000000000000{}000000000000000000000000{}{}{}",
            deal_id_raw, payment_token_raw, price_per_epoch_raw, required_stake_raw
        );

        let result = parse_chain_deal_data(data);
        assert!(result.is_ok());
        let deal_data = result.unwrap();
        assert_eq!(deal_data.deal_id, deal_id_expected);
        assert_eq!(deal_data.payment_token, payment_token_expected);
        assert_eq!(deal_data.price_per_epoch, price_per_epoch_expected);
        assert_eq!(deal_data.required_stake, required_stake_expected);
    }

    #[test]
    fn test_fail_empty() {
        let data = String::new();
        let result = parse_chain_deal_data(data);
        assert!(result.is_err());
        assert_matches!(result, Err(DealParseError::WrongLength { actual, .. }) if actual == 0);
    }

    #[test]
    fn test_fail_something() {
        let data = String::from("1234567890");
        let result = parse_chain_deal_data(data);
        assert!(result.is_err());
        assert_matches!(result, Err(DealParseError::WrongLength { .. }));
    }
}

use marine_rs_sdk::marine;

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
pub fn parse_chain_deal_data(data: String) -> DealData {
    // Each field is 32 bytes, one byte is represented by two hex symbols.
    let field_len = 64;

    // truncate leading "0x"
    let (_, rest) = data.split_at(2);
    let (deal_id_raw, rest) = rest.split_at(field_len);
    let (payment_token_raw, rest) = rest.split_at(field_len);
    let (price_per_epoch, required_stake) = rest.split_at(field_len);

    // Real length of `deal_id` and `payment_token` is 20 bytes.
    let truncate_len = field_len - 40;
    let (_, deal_id) = deal_id_raw.split_at(truncate_len);
    let (_, payment_token) = payment_token_raw.split_at(truncate_len);

    DealData {
        deal_id: format!("0x{}", deal_id),
        payment_token: format!("0x{}", payment_token),
        price_per_epoch: format!("0x{}", price_per_epoch),
        required_stake: format!("0x{}", required_stake),
    }
}

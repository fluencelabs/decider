use crate::chain::deal_created::DealCreated;
use marine_rs_sdk::marine;

#[marine]
pub struct DealCreatedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealCreated>,
    to_block: String,
}

impl DealCreatedResult {
    pub fn ok(result: Vec<DealCreated>, to_block: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
            to_block,
        }
    }

    pub fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
            to_block: String::new(),
        }
    }
}

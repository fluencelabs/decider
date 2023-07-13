use marine_rs_sdk::marine;

use crate::chain::deal_created::DealCreated;

#[marine]
pub struct DealCreatedResult {
    error: Vec<String>,
    success: bool,
    result: Vec<DealCreated>,
    /// The response contains logs for blocks from `left_boundary` to `right_boundary`
    right_boundary: String,
}

impl DealCreatedResult {
    pub fn ok(result: Vec<DealCreated>, right_boundary: String) -> Self {
        Self {
            success: true,
            error: vec![],
            result,
            right_boundary,
        }
    }

    pub fn error(err_msg: String) -> Self {
        Self {
            success: false,
            error: vec![err_msg],
            result: vec![],
            right_boundary: String::new(),
        }
    }
}

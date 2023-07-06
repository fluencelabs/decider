use marine_rs_sdk::marine;

use crate::chain::chain_data::ChainData;
use crate::chain::deal_changed::{DealChanged, DealChangedData};
use crate::hex::{hex_to_int, int_to_hex};
use crate::jsonrpc::get_logs::GetLogsReq;
use crate::jsonrpc::JsonRpcReq;

// TODO: make it configurable
const DEFAULT_BLOCK_RANGE: u64 = 500;

#[derive(Debug)]
#[marine]
/// RENAME: why is it called `DealUpate`?
pub struct DealUpdate {
    pub deal_info: DealInfo,
    pub left_boundary: String,
}

/// Default value for `right_boundary` in chain polling
///
/// Calculated based on `left_boundary` by adding `DEFAULT_BLOCK_RANGE`
/// If `left_boundary` is not a hex string, return `"latest"`
pub fn default_right_boundary(left_boundary: &str) -> String {
    let right_boundary = try {
        let left_boundary = hex_to_int(left_boundary)?;
        left_boundary.checked_add(DEFAULT_BLOCK_RANGE)?
    };
    match right_boundary {
        Some(right_boundary) => int_to_hex(right_boundary),
        None => "latest".to_string(),
    }
}

#[derive(Debug)]
#[marine]
pub struct DealInfo {
    pub worker_id: String,
    pub deal_id: String,
}

#[marine]
pub struct DealChangedResult {
    pub success: bool,
    /// optional error
    pub error: Vec<String>,
    /// optional result (present if success is true)
    /// Optionals in AIR and marine are represented with Vec
    /// Some(x) is [x], None is []
    pub result: Vec<DealChanged>,
    /// The response contains logs for blocks from `left_boundary` to `right_boundary`
    pub right_boundary: String,
    /// Return chain info to be able to find which chain to update
    pub deal_info: DealInfo,
}

impl DealChangedResult {
    pub fn ok(right_boundary: String, deal_info: DealInfo, change: DealChanged) -> Self {
        Self {
            success: true,
            error: vec![],
            // Optionals in AIR and marine are represented with Vec. Some(x) is [x]
            result: vec![change],
            right_boundary,
            deal_info,
        }
    }

    pub fn error(right_boundary: String, deal_info: DealInfo, err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            // Optionals in AIR and marine are represented with Vec. None is []
            result: vec![],
            right_boundary,
            deal_info,
        }
    }
}

#[marine]
pub struct MultipleDealsChanged {
    pub changes: Vec<DealChangedResult>,
    pub success: bool,
    pub error: Vec<String>,
}

impl MultipleDealsChanged {
    pub fn empty() -> Self {
        Self::ok(<_>::default())
    }

    pub fn ok(result: Vec<DealChangedResult>) -> Self {
        Self {
            success: true,
            error: vec![],
            changes: result,
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            changes: vec![],
        }
    }
}

pub fn deal_changed_req(deal: &DealUpdate, idx: usize) -> JsonRpcReq<GetLogsReq> {
    let right_boundary = default_right_boundary(&deal.left_boundary);
    let address = format!("0x{}", deal.deal_info.deal_id);
    let req = GetLogsReq {
        address,
        topics: vec![DealChangedData::topic()],
        from_block: deal.left_boundary.clone(),
        to_block: right_boundary,
    };

    req.to_jsonrpc(idx as u32)
}

pub fn deal_changed_req_batch(deals: &[DealUpdate]) -> Vec<JsonRpcReq<GetLogsReq>> {
    deals
        .iter()
        .enumerate()
        .map(|(idx, deal)| deal_changed_req(deal, idx))
        .collect::<Vec<_>>()
}

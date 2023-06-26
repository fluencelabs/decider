use marine_rs_sdk::marine;

use crate::chain::deal_changed::{DealChanged, DealChangedData};
use crate::chain::JsonRpcReq;
use crate::hex::{hex_to_int, int_to_hex};
use crate::jsonrpc::get_logs::GetLogsReq;

const DEFAULT_BLOCK_RANGE: u64 = 9999;

#[derive(Debug)]
#[marine]
/// RENAME: why is it called `DealUpate`?
pub struct DealUpdate {
    pub deal_info: DealInfo,
    pub from_block: String,
}

/// Default value for `to_block` in chain polling
///
/// Calculated based on `from_block` by adding `DEFAULT_BLOCK_RANGE`
/// If `from_block` is not a hex string, return `"latest"`
pub fn default_to_block(from_block: &str) -> String {
    let to_block = try {
        let from_block = hex_to_int(from_block)?;
        from_block.checked_add(DEFAULT_BLOCK_RANGE)?
    };
    match to_block {
        Some(to_block) => int_to_hex(to_block),
        None => "latest".to_string(),
    }
}

#[derive(Debug)]
#[marine]
pub struct DealInfo {
    pub worker_id: String,
    pub deal_id: String,
}

/// Optionals in AIR and marine are represented with Vec
/// Some(x) is [x], None is []
type AirOption<T> = Vec<T>;

#[marine]
pub struct DealChangedResult {
    pub success: bool,
    /// optional error
    pub error: Vec<String>,
    /// optional result (present if success is true)
    pub result: AirOption<DealChanged>,
    /// The request checked blocks from `from_block` to `to_block`
    pub to_block: String,
    /// Return chain info to be able to find which chain to update
    pub deal_info: DealInfo,
}

impl DealChangedResult {
    pub fn ok(to_block: String, deal_info: DealInfo, change: DealChanged) -> Self {
        Self {
            success: true,
            error: vec![],
            // Optionals in AIR and marine are represented with Vec. Some(x) is [x]
            result: vec![change],
            to_block,
            deal_info,
        }
    }

    pub fn error(to_block: String, deal_info: DealInfo, err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            // Optionals in AIR and marine are represented with Vec. None is []
            result: vec![],
            to_block,
            deal_info,
        }
    }
}

#[marine]
pub struct MultipleDealsChanged {
    pub result: Vec<DealChangedResult>,
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
            result,
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            result: vec![],
        }
    }
}

pub fn deal_changed_req(deal: &DealUpdate, idx: usize) -> JsonRpcReq<Vec<GetLogsReq>> {
    let to_block = default_to_block(&deal.from_block);
    let address = format!("0x{}", deal.deal_info.deal_id);
    let req = GetLogsReq {
        address,
        topics: vec![DealChangedData::topic()],
        from_block: deal.from_block.clone(),
        to_block,
    };

    req.to_jsonrpc(idx as u32)
}

pub fn deal_changed_req_batch(deals: &[DealUpdate]) -> Vec<JsonRpcReq<Vec<GetLogsReq>>> {
    deals
        .iter()
        .enumerate()
        .map(|(idx, deal)| deal_changed_req(deal, idx))
        .collect::<Vec<_>>()
}

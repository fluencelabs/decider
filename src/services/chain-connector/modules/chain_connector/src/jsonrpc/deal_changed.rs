use marine_rs_sdk::marine;

use crate::chain::chain_data::ChainData;
use crate::chain::deal_changed::{DealChanged, DealChangedData};
use crate::jsonrpc::get_logs::GetLogsReq;
use crate::jsonrpc::right_boundary::default_right_boundary;
use crate::jsonrpc::JsonRpcReq;

#[derive(Debug)]
#[marine]
pub struct DealChangesReq {
    pub deal_info: DealInfo,
    pub left_boundary: String,
}

impl DealChangesReq {
    pub fn jsonrpc(&self, idx: usize) -> JsonRpcReq<GetLogsReq> {
        let right_boundary = default_right_boundary(&self.left_boundary);
        let req = GetLogsReq {
            address: self.deal_info.deal_id.clone(),
            topics: vec![DealChangedData::topic()],
            from_block: self.left_boundary.clone(),
            to_block: right_boundary,
        };

        req.to_jsonrpc(idx as u32)
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
    /// optional logs (present if success is true)
    /// Optionals in AIR and marine are represented with Vec
    /// Some(x) is [x], None is []
    pub log: Vec<DealChanged>,
    /// The response contains logs for blocks from `left_boundary` to `right_boundary`
    pub right_boundary: String,
    /// Return chain info to be able to find which chain to update
    pub deal_info: DealInfo,
}

impl DealChangedResult {
    /// Allow empty changes
    pub fn ok(right_boundary: String, deal_info: DealInfo, change: Option<DealChanged>) -> Self {
        Self {
            success: true,
            error: vec![],
            // Optionals in AIR and marine are represented with Vec. Some(x) is [x]
            log: change.map(|change| vec![change]).unwrap_or_default(),
            right_boundary,
            deal_info,
        }
    }

    pub fn error(right_boundary: String, deal_info: DealInfo, err: String) -> Self {
        Self {
            success: false,
            error: vec![err],
            // Optionals in AIR and marine are represented with Vec. None is []
            log: vec![],
            right_boundary,
            deal_info,
        }
    }
}
